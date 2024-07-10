use borsh::{BorshDeserialize, BorshSerialize};
use l1x_sdk::{
    call_contract,
    contract_interaction::ContractCall,
    gas_left,
    types::{Address, Gas},
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
pub struct NameServiceContract {
    address: Address,
}

impl NameServiceContract {
    pub fn new(address: Address) -> Self {
        Self { address }
    }

    pub fn list_owned_names(&self, user: Address) -> Vec<Name> {
        let args = {
            #[derive(Serialize)]
            struct Args {
                user: Address,
            }
            serde_json::to_vec(&Args { user }).unwrap()
        };
        self.call(function!(), args, gas_left().saturating_sub(10_000))
    }

    pub fn register_root_name(&mut self, name: String, resolver: Option<Address>) {
        let args = {
            #[derive(Serialize)]
            struct Args {
                name: String,
                resolver: Option<Address>,
            }
            serde_json::to_vec(&Args { name, resolver }).unwrap()
        };
        self.call_mut_empty(function!(), args, gas_left().saturating_sub(10_000))
    }

    pub fn transfer_name(&mut self, name: String, new_owner: Address) {
        let args = {
            #[derive(Serialize)]
            struct Args {
                name: String,
                new_owner: Address,
            }
            serde_json::to_vec(&Args { name, new_owner }).unwrap()
        };
        self.call_mut_empty(function!(), args, gas_left().saturating_sub(10_000))
    }

    pub fn change_name_resolver(&mut self, name: String, new_resolver: Address) {
        let args = {
            #[derive(Serialize)]
            struct Args {
                name: String,
                new_resolver: Address,
            }
            serde_json::to_vec(&Args { name, new_resolver }).unwrap()
        };
        self.call_mut_empty(function!(), args, gas_left().saturating_sub(10_000))
    }

    fn call<R>(&self, method_name: &str, args: Vec<u8>, fee_limit: Gas) -> R
    where
        for<'a> R: Deserialize<'a>,
    {
        let mut result = None;
        self.call_internal(method_name, args, fee_limit, true, &mut result);
        result.expect("The external contract returned the empty result")
    }

    fn call_mut_empty(&self, method_name: &str, args: Vec<u8>, fee_limit: Gas) {
        let mut result = None;
        self.call_internal::<()>(method_name, args, fee_limit, false, &mut result);
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