use borsh::{BorshDeserialize, BorshSerialize};
use l1x_sdk::{
    call_contract,
    contract_interaction::ContractCall,
    gas_left,
    types::{Address, Gas},
};
use serde::{Deserialize, Serialize};

use crate::ScoreBoardMetadata;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct ScoreBoardContract {
    address: Address,
}

impl ScoreBoardContract {
    pub fn new(address: Address) -> Self {
        Self { address }
    }

    pub fn start_game(&mut self, users: Vec<Address>) {
        let args = {
            #[derive(Serialize)]
            struct Args {
                users: Vec<Address>,
            }
            serde_json::to_vec(&Args { users }).unwrap()
        };
        self.call_mut_empty("start_game", args, gas_left().saturating_sub(10_000))
    }

    pub fn end_game(&mut self) {
        self.call_mut_empty("end_game", "{}".to_string().into_bytes(), gas_left().saturating_sub(10_000))
    }

    pub fn metadata(&self) -> ScoreBoardMetadata {
        self.call("metadata", "{}".to_string().into_bytes(), gas_left().saturating_sub(10_000))
    }

    pub fn administrator(&self) -> Address {
        self.call("administrator", "{}".to_string().into_bytes(), gas_left().saturating_sub(10_000))
    }

    pub fn users(&self) -> Vec<Address> {
        self.call("users", "{}".to_string().into_bytes(), gas_left().saturating_sub(10_000))
    }

    fn call<R>(&self, method_name: &str, args: Vec<u8>, gas_limit: Gas) -> R
    where
        for<'a> R: Deserialize<'a>,
    {
        let mut result = None;
        self.call_internal(method_name, args, gas_limit, true, &mut result);
        result.expect("The external contract returned the empty result")
    }

    fn call_mut_empty(&self, method_name: &str, args: Vec<u8>, gas_limit: Gas) {
        let mut result = None;
        self.call_internal::<()>(method_name, args, gas_limit, false, &mut result);
        assert!(result.is_none(), "The external contract returned something but that was not expected");
    }

    fn call_internal<R>(
        &self,
        method_name: &str,
        args: Vec<u8>,
        gas_limit: Gas,
        read_only: bool,
        result: &mut Option<R>,
    ) where
        for<'a> R: Deserialize<'a>,
    {
        let call = ContractCall {
            contract_address: self.address.clone(),
            method_name: method_name.to_string(),
            args,
            read_only,
            gas_limit,
        };

        match call_contract(&call) {
            Ok(res) => {
                if !res.is_empty() {
                    let res: Result<R, serde_json::Error> = serde_json::from_slice(&res);
                    // l1x_sdk::msg(&format!("Returned by the external contract: {:?}", res));

                    let res = res.expect("Can't deserialize external contract's return value");
                    *result = Some(res);
                }
            }
            Err(e) => {
                panic!("Got the error when called contract {}, {:?}", self.address.clone(), e);
            }
        }
    }
}