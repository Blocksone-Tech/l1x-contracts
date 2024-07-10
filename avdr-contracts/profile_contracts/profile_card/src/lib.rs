use std::collections::BTreeMap;

use borsh::{BorshDeserialize, BorshSerialize};
use l1x_sdk::{
    caller_address, contract, contract_instance_address, contract_owner_address,
    types::{Address, U128},
};
use serde::{Deserialize, Serialize};
use types::nft_interface::NftContract;

/// Key for the storage of the contract data.
const STORAGE_CONTRACT_KEY: &[u8] = b"state";

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
pub struct Nft {
    address: Address,
    id: U128,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Clone)]
pub struct DomainName {
    name: String,
    name_nft: Nft,
    provatar_nft: Option<Nft>,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Metadata {
    name: String,
    description: Option<String>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    metadata: Metadata,
    names: BTreeMap<String, DomainName>,
}

#[contract]
impl Contract {
    pub fn new(metadata: Metadata) {
        Self::assert_if_initialized();
        Self::assert_owner();

        let mut contract = Self { names: BTreeMap::new(), metadata };
        contract.save();
    }

    pub fn add_name(name_nft: Nft, provatar_nft: Option<Nft>) {
        Self::assert_owner();

        let mut contract = Self::load();
        contract.add_name_internal(name_nft, provatar_nft);
        contract.save();
    }

    pub fn add_provatar(name: String, provatar_nft: Nft) {
        Self::assert_owner();

        let mut contract = Self::load();
        contract.add_provatar_internal(name, provatar_nft);
        contract.save();
    }

    pub fn transfer_name(name: String, to: Address) {
        Self::assert_owner();

        let mut contract = Self::load();
        contract.transfer_name_internal(name, to);
        contract.save();
    }

    pub fn update_metadata(metadata: Metadata) {
        Self::assert_owner();

        let mut contract = Self::load();
        contract.update_metadata_internal(metadata);
        contract.save();
    }

    pub fn list_names() -> Vec<DomainName> {
        let contract = Self::load();
        contract.list_names_internal()
    }

    pub fn metadata() -> Metadata {
        let contract = Self::load();
        contract.metadata
    }
}

impl Contract {
    fn add_name_internal(&mut self, name_nft: Nft, proavtar_nft: Option<Nft>) {
        let mut nft_contract = NftContract::new(name_nft.address);
        let caller = caller_address();

        assert_eq!(nft_contract.nft_owner_of(name_nft.id), caller, "The caller doesn't own Name NFT");

        nft_contract.nft_transfer_from(caller, contract_instance_address(), name_nft.id);

        let domain_name = nft_contract.nft_token_name(name_nft.id);
        self.names.insert(domain_name.clone(), DomainName { name: domain_name.clone(), name_nft, provatar_nft: None });

        if let Some(proavtar_nft) = proavtar_nft {
            self.add_provatar_nft(domain_name, proavtar_nft);
        }
    }

    fn add_provatar_internal(&mut self, name: String, proavtar_nft: Nft) {
        self.add_provatar_nft(name, proavtar_nft);
    }

    fn add_provatar_nft(&mut self, name: String, proavtar_nft: Nft) {
        let name_rec = self.names.get_mut(&name).expect("Can't find the specified name");
        let mut nft_contract = NftContract::new(proavtar_nft.address);
        let caller = caller_address();

        assert_eq!(nft_contract.nft_owner_of(proavtar_nft.id), caller, "The caller doesn't own Provatar NFT");

        nft_contract.nft_transfer_from(caller, contract_instance_address(), proavtar_nft.id);

        name_rec.provatar_nft = Some(proavtar_nft);
    }

    fn transfer_name_internal(&mut self, name: String, to: Address) {
        let name_rec = self.names.remove(&name).expect("Can't find the specified name");

        let mut nft_contract = NftContract::new(name_rec.name_nft.address);
        nft_contract.nft_transfer_from(contract_instance_address(), to, name_rec.name_nft.id);
        if let Some(provatar_nft) = name_rec.provatar_nft {
            let mut nft_contract = NftContract::new(provatar_nft.address);
            nft_contract.nft_transfer_from(contract_instance_address(), to, provatar_nft.id);
        }
    }

    fn update_metadata_internal(&mut self, metadata: Metadata) {
        self.metadata = metadata
    }

    fn list_names_internal(&self) -> Vec<DomainName> {
        self.names.values().cloned().collect()
    }

    fn assert_if_initialized() {
        assert!(l1x_sdk::storage_read(STORAGE_CONTRACT_KEY).is_none(), "The contract is already initialized");
    }

    fn assert_owner() {
        assert_eq!(caller_address(), contract_owner_address(), "This function can be called only by the owner");
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