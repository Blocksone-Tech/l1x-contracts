use crate::{Content, Metadata, PublicKey};
use l1x_sdk::{
    call_contract,
    contract_interaction::ContractCall,
    gas_left,
    types::{Address, Gas},
};
use serde::Deserialize;

pub struct DataStorageContract {
    address: Address,
}

impl DataStorageContract {
    pub fn new(address: Address) -> Self {
        DataStorageContract { address }
    }

    pub fn content(&self) -> Content {
        self.call("content", "{}".to_string().into_bytes(), gas_left().saturating_sub(10_000))
    }

    pub fn name(&self) -> String {
        self.call("name", "{}".to_string().into_bytes(), gas_left().saturating_sub(10_000))
    }

    pub fn description(&self) -> String {
        self.call("description", "{}".to_string().into_bytes(), gas_left().saturating_sub(10_000))
    }

    pub fn public_key(&self) -> PublicKey {
        self.call("public_key", "{}".to_string().into_bytes(), gas_left().saturating_sub(10_000))
    }

    pub fn encrypted(&self) -> bool {
        self.call("encrypted", "{}".to_string().into_bytes(), gas_left().saturating_sub(10_000))
    }

    pub fn metadata(&self) -> Metadata {
        self.call("metadata", "{}".to_string().into_bytes(), gas_left().saturating_sub(10_000))
    }

    fn call<R>(&self, method_name: &str, args: Vec<u8>, gas_limit: Gas) -> R
    where
        for<'a> R: Deserialize<'a>,
    {
        let call = ContractCall {
            contract_address: self.address.clone(),
            method_name: method_name.to_string(),
            args,
            read_only: true,
            gas_limit,
        };

        match call_contract(&call) {
            Ok(res) => {
                if res.is_empty() {
                    panic!("The external contract returned the empty result");
                } else {
                    let res: Result<R, serde_json::Error> = serde_json::from_slice(&res);
                    // l1x_sdk::msg(&format!("Returned by the external contract: {:?}", res));

                    res.expect("Can't deserialize external contract's return value")
                }
            }
            Err(e) => {
                panic!("Got the error when called contract {}, {:?}", self.address.clone(), e);
            }
        }
    }
}