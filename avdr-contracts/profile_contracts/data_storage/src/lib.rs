use borsh::{BorshDeserialize, BorshSerialize};
use l1x_sdk::{caller_address, contract, contract_owner_address, store::Vector, types::Address};
use types::{Content, Metadata, PublicKey};

const STORAGE_CONTRACT_KEY: &[u8] = b"STATE";
const STORAGE_CONTENT_KEY: &[u8] = b"CONTENT";

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    metadata: Metadata,
    content: Vector<Content>,
}

#[contract]
impl Contract {
    pub fn new(name: String, description: Option<String>, pub_key: PublicKey, content: Content) {
        let caller = caller_address();
        assert_eq!(caller, contract_owner_address(), "Only the owner can call this function");
        Self::assert_if_initialized();

        let mut contract = Self {
            metadata: Metadata { name, description, pub_key, owner: caller, versioned: false },
            content: Vector::new(STORAGE_CONTENT_KEY.to_vec()),
        };
        contract.content.push(content);
        contract.content.flush();
        contract.save()
    }

    pub fn content() -> Content {
        let contract = Self::load();
        contract.content.get(0).expect("Can't read the content").clone()
    }

    pub fn name() -> String {
        let contract = Self::load();
        contract.metadata.name
    }

    pub fn description() -> String {
        let contract = Self::load();
        contract.metadata.description.expect("No description")
    }

    pub fn public_key() -> PublicKey {
        let contract = Self::load();
        contract.metadata.pub_key
    }

    pub fn encrypted() -> bool {
        let contract = Self::load();
        !contract.metadata.pub_key.0.is_empty()
    }

    pub fn owner() -> Address {
        let contract = Self::load();
        contract.metadata.owner
    }

    pub fn metadata() -> Metadata {
        let contract = Self::load();
        contract.metadata
    }

    fn assert_if_initialized() {
        assert!(l1x_sdk::storage_read(STORAGE_CONTRACT_KEY).is_none(), "The contract is already initialized");
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