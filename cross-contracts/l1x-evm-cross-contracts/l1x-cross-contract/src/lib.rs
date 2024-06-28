use borsh::BorshDeserialize;
use borsh::BorshSerialize;
use l1x_sdk::call_contract;
use l1x_sdk::contract;
use l1x_sdk::contract_interaction::ContractCall;
use l1x_sdk::types::{Address, U128};
use solabi;

const STORAGE_CONTRACT_KEY: &[u8] = b"STATE";

#[derive(Debug, BorshSerialize, BorshDeserialize)]
struct EvmErc20 {
    evm_contract_address: Address,
}

impl EvmErc20 {
    pub fn new(evm_contract_address: Address) -> Self {
        Self {
            evm_contract_address,
        }
    }

    pub fn get_value(&self) -> solabi::U256 {
        let func: solabi::FunctionEncoder<(), (solabi::U256,)> =
            solabi::FunctionEncoder::new(solabi::selector!("getValue()"));

        self.call_evm(&func, &(), true).0
    }

  
    pub fn set_value(&self, _data: solabi::U256) -> bool {
        let func: solabi::FunctionEncoder<solabi::U256, (bool,)> =
            solabi::FunctionEncoder::new(solabi::selector!("setValue(uint256)"));

        self.call_evm(&func, &(_data), false).0
    }

   

    fn call_evm<P, R>(&self, func: &solabi::FunctionEncoder<P, R>, params: &P, read_only: bool) -> R
    where
        P: solabi::encode::Encode + solabi::decode::Decode,
        R: solabi::encode::Encode + solabi::decode::Decode,
    {
        let args = func.encode_params(params);

        l1x_sdk::msg(&format!("L1XVM: HEX_ARG: {}", hex::encode(&args)));

        let call = ContractCall {
            contract_address: self.evm_contract_address.clone(),
            method_name: "".to_string(), // method_name is not used in case of EVM call
            args,
            read_only,
            gas_limit: 12,
        };

        let ret = call_contract(&call).expect("Function returned nothing");

        func.decode_returns(&ret)
            .unwrap_or_else(|e| panic!("err: {}", e.to_string()))
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Contract {
    evm_erc20: EvmErc20,
}

#[contract]
impl Contract {
    pub fn new(evm_address: Address) {
        let mut contract = Self {
            evm_erc20: EvmErc20::new(evm_address),
        };
        Self::save(&mut contract);
    }

    pub fn get_value() -> String{
        l1x_sdk::msg(&format!("L1XVM: getValue"));

        let contract = Self::load();
        let ret = contract
            .evm_erc20
            .get_value()
            .to_string();

        l1x_sdk::msg(&format!("L1XVM: getValue returns {}",  ret));
        ret
    }

    pub fn set_value(data: U128) {
        l1x_sdk::msg(&format!(
            "L1XVM: setValue init {:?}", data
        ));

        let contract = Self::load();

        contract.evm_erc20.set_value(
            solabi::U256::from(data.0),
        );
        l1x_sdk::msg(&format!(
            "L1XVM: setValue done {:?}", data
        ));
    }

    fn load() -> Self {
        match l1x_sdk::storage_read(STORAGE_CONTRACT_KEY) {
            Some(bytes) => Self::try_from_slice(&bytes).unwrap(),
            None => panic!("The contract isn't initialized"),
        }
    }

    fn save(&mut self) {
        l1x_sdk::storage_write(STORAGE_CONTRACT_KEY, &self.try_to_vec().unwrap());
    }
}
