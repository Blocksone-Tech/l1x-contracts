use crate::Cid;
use borsh::{BorshDeserialize, BorshSerialize};
use l1x_sdk::{
    call_contract,
    contract_interaction::ContractCall,
    gas_left,
    types::{Address, Gas, U64},
};
use serde::{Deserialize, Serialize};
use std::hash::{Hash, Hasher};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct ReqIdx(pub U64);

// l1x-sdk doesn't derive Hash for U64.
// TODO: Remove this when l1x-sdk will derive Hash for U64
impl Hash for ReqIdx {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0 .0.hash(state);
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub enum Permission {
    Write,
    Remove,
    Approve,
    Reject,
    Revoke,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct AccessControlContract {
    address: Address,
}

impl AccessControlContract {
    pub fn new(address: Address) -> Self {
        Self { address }
    }

    pub fn permissions(&self, user: Address, cid: Cid) -> Vec<Permission> {
        let args = {
            #[derive(Serialize)]
            struct Args {
                user: Address,
                cid: Cid,
            }
            serde_json::to_vec(&Args { user, cid }).unwrap()
        };
        self.call("permissions", args, gas_left().saturating_sub(10_000))
    }

    pub fn has_perm(&self, user: Address, cid: Cid, perm: Permission) -> bool {
        let args = {
            #[derive(Serialize)]
            struct Args {
                user: Address,
                cid: Cid,
                perm: Permission,
            }
            serde_json::to_vec(&Args { user, cid, perm }).unwrap()
        };
        self.call("has_perm", args, gas_left().saturating_sub(10_000))
    }
    pub fn has_approved_share(&self, cid: Cid, from_cid: Cid) -> bool {
        let args = {
            #[derive(Serialize)]
            struct Args {
                cid: Cid,
                from_cid: Cid,
            }
            serde_json::to_vec(&Args { cid, from_cid }).unwrap()
        };
        self.call("has_approved_share", args, gas_left().saturating_sub(10_000))
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