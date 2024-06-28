use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use l1x_sdk::call_contract;
use l1x_sdk::contract;
use l1x_sdk::contract_interaction::ContractCall;
use l1x_sdk::emit_event_experimental;
use l1x_sdk::types::Address;
use serde::Serialize;

const STORAGE_CONTRACT_KEY: &[u8] = b"STATE";

#[derive(BorshSerialize)]
struct Event {
    name: String,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    contract_instance_address: Address,
}

#[contract]
impl Contract {
    fn load() -> Self {
        match l1x_sdk::storage_read(STORAGE_CONTRACT_KEY) {
            Some(bytes) => Self::try_from_slice(&bytes).unwrap(),
            None => panic!("The contract isn't initialized"),
        }
    }

    fn save(&mut self) {
        l1x_sdk::storage_write(STORAGE_CONTRACT_KEY, &self.try_to_vec().unwrap());
    }

    pub fn new(contract_instance_address: Option<Address>) {
        let mut state = if let Some(contract_instance_address) = contract_instance_address {
            Self {
                contract_instance_address,
            }
        } else {
            let test_address = b"l1x_contract\0\0\0\0\0\0\0\0";
            Self {
                contract_instance_address: Address::try_from(test_address.to_vec()).unwrap(),
            }
        };

        state.save()
    }

    pub fn hello() {
        let address = Self::load().contract_instance_address;
        let call = ContractCall {
            contract_address: address.clone(),
            method_name: "get_names".to_string(),
            args: "{}".as_bytes().to_vec(),
            read_only: true,
            gas_limit: 10000,
        };

        match call_contract(&call) {
            Ok(res) => {
                let res: Result<Vec<String>, serde_json::Error> = serde_json::from_slice(&res);
                l1x_sdk::msg(&format!("Returned by the external contract: {:?}", res));
            }
            Err(e) => {
                l1x_sdk::msg(&format!("The external contract failed with the error: {e}"));
            }
        }
    }

    pub fn add_name(name: String) {
        let address = Self::load().contract_instance_address;

        let args = {
            #[derive(Serialize)]
            struct Args {
                name: String,
            }
            Args { name }
        };

        let call = ContractCall {
            contract_address: address.clone(),
            method_name: "add_name".to_string(),
            args: serde_json::to_vec(&args).unwrap(),
            read_only: false,
            gas_limit: 30000,
        };

        match call_contract(&call) {
            Ok(res) => {
                if res.is_empty() {
                    l1x_sdk::msg(&format!("The external contract returned the empty result"));
                } else {
                    let res: Result<Vec<String>, serde_json::Error> = serde_json::from_slice(&res);
                    l1x_sdk::msg(&format!("Returned by the external contract: {:?}", res));
                }
            }
            Err(e) => {
                l1x_sdk::msg(&format!("The external contract failed with the error: {e}"));
            }
        }
    }

    pub fn emit_event() {
        emit_event_experimental(Event {
            name: "Hello".to_string(),
        });
    }
}
