use borsh::{BorshDeserialize, BorshSerialize};
use l1x_sdk::types::{Address, U64};
use l1x_sdk::{caller_address, contract, contract_instance_address, contract_owner_address};
use serde::{Deserialize, Serialize};
use types::access_control_interface::{AccessControlContract, Permission};
use types::data_storage_interface::DataStorageContract;
use types::{Cid, Metadata, PublicKey};

const STORAGE_CONTRACT_KEY: &[u8] = b"STATE";

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub enum DataInfo {
    Content(Cid),
    Removed,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct SharedDataInfo {
    cid: Cid,
    from_version: U64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    versions: Vec<DataInfo>,
    shared: Vec<SharedDataInfo>,
    access_control: AccessControlContract,
    metadata: Metadata,
}

#[contract]
impl Contract {
    pub fn new(access_control_address: Address, cid: Cid) {
        assert_eq!(caller_address(), contract_owner_address(), "Only the owner can call this function");
        Self::assert_if_initialized();

        let data_storage = DataStorageContract::new(cid.0);
        let mut metadata = data_storage.metadata();
        metadata.versioned = true;

        Self {
            versions: vec![DataInfo::Content(cid)],
            shared: Vec::new(),
            access_control: AccessControlContract::new(access_control_address),
            metadata,
        }
        .save();
    }

    pub fn submit(cid: Cid) {
        let mut contract = Self::load();
        contract.submit_internal(cid);
        contract.save();
    }

    pub fn remove() {
        let mut contract = Self::load();
        contract.remove_internal();
        contract.save();
    }

    pub fn submit_shared(cid: Cid, from_cid: Cid) {
        let mut contract = Self::load();
        contract.submit_shared_internal(cid, from_cid);
        contract.save();
    }

    pub fn get() -> Vec<DataInfo> {
        let contract = Self::load();
        contract.get_internal()
    }

    pub fn get_version(version: U64) -> DataInfo {
        let contract = Self::load();
        contract.get_version_internal(version)
    }

    pub fn get_newest() -> DataInfo {
        let contract = Self::load();
        contract.get_newest_internal()
    }

    pub fn get_shared() -> Vec<SharedDataInfo> {
        let contract = Self::load();
        contract.get_shared_internal()
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

#[contract]
impl Contract {
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

    pub fn owner() -> Address {
        let contract = Self::load();
        contract.metadata.owner
    }

    pub fn metadata() -> Metadata {
        let contract = Self::load();
        contract.metadata
    }
}

impl Contract {
    fn submit_internal(&mut self, cid: Cid) {
        assert!(
            self.access_control.has_perm(caller_address(), Cid(contract_instance_address()), Permission::Write),
            "No Write permissions"
        );

        let data_storage = DataStorageContract::new(cid.0);
        let cid_metadata = data_storage.metadata();
        assert_eq!(self.metadata.owner, cid_metadata.owner, "Owners are not equal");
        assert_eq!(self.metadata.name, cid_metadata.name, "Names are not equal");
        self.versions.push(DataInfo::Content(cid));
    }

    fn remove_internal(&mut self) {
        assert!(
            self.access_control.has_perm(caller_address(), Cid(contract_instance_address()), Permission::Remove),
            "No Remove permissions"
        );
        self.versions.push(DataInfo::Removed);
    }

    fn submit_shared_internal(&mut self, cid: Cid, from_cid: Cid) {
        assert!(
            self.access_control.has_perm(caller_address(), Cid(contract_instance_address()), Permission::Write),
            "No Write permissions"
        );
        assert!(
            self.access_control.has_approved_share(cid.clone(), from_cid.clone()),
            "The share request is not approved"
        );

        let from_version = self
            .versions
            .iter()
            .position(|d| if let DataInfo::Content(c) = d { *c == from_cid } else { false })
            .expect("Can't find the version of from_cid") as u64;

        self.shared.push(SharedDataInfo { cid, from_version: from_version.into() })
    }

    pub fn get_internal(&self) -> Vec<DataInfo> {
        self.versions.clone()
    }

    pub fn get_version_internal(&self, version: l1x_sdk::types::U64) -> DataInfo {
        self.versions.get(version.0 as usize).expect("There is not such version of the data").clone()
    }

    pub fn get_newest_internal(&self) -> DataInfo {
        self.versions.last().unwrap().clone()
    }

    pub fn get_shared_internal(&self) -> Vec<SharedDataInfo> {
        self.shared.clone()
    }
}