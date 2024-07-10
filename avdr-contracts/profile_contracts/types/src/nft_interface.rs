use borsh::{BorshDeserialize, BorshSerialize};
use l1x_sdk::{
    call_contract,
    contract_interaction::ContractCall,
    gas_left,
    types::{Address, Gas, U128},
};
use serde::{Deserialize, Serialize};

macro_rules! function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

#[derive(Deserialize)]
pub struct Name {
    pub name: String,
    pub resolver: Address,
    pub owner: Address,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct NftContract {
    address: Address,
}

impl NftContract {
    pub fn new(address: Address) -> Self {
        Self { address }
    }

    pub fn nft_transfer_from(&mut self, from: Address, to: Address, id: U128) {
        let args = {
            #[derive(Serialize)]
            struct Args {
                from: Address,
                to: Address,
                id: U128,
            }
            serde_json::to_vec(&Args { from, to, id }).unwrap()
        };
        self.call_mut_empty(function!(), args, gas_left().saturating_sub(10_000))
    }

    pub fn nft_owner_of(&self, id: U128) -> Address {
        let args = {
            #[derive(Serialize)]
            struct Args {
                id: U128,
            }
            serde_json::to_vec(&Args { id }).unwrap()
        };
        self.call(function!(), args, gas_left().saturating_sub(10_000))
    }

    pub fn nft_token_name(&self, id: U128) -> String {
        let args = {
            #[derive(Serialize)]
            struct Args {
                id: U128,
            }
            serde_json::to_vec(&Args { id }).unwrap()
        };
        self.call(function!(), args, gas_left().saturating_sub(10_000))
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