use std::collections::{BTreeMap, BTreeSet};

use borsh::{BorshDeserialize, BorshSerialize};
use l1x_sdk::store::LookupMap;
use l1x_sdk::types::Address;
use l1x_sdk::{caller_address, contract, contract_owner_address};
use serde::Serialize;

const STORAGE_CONTRACT_KEY: &[u8] = b"STATE";
const STORAGE_RESOLVE_NAME_KEY: &[u8] = b"RESOLVE_NAME";
const STORAGE_OWNED_NAMES_KEY: &[u8] = b"OWNED_NAMES";
const ROOT_DOMAIN: &str = "l1x";

/// Max length for NFT name.
const MAX_NAME_LENGTH: usize = 20;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Clone)]
pub struct Name {
    pub name: String,
    pub resolver: Address,
    pub owner: Address,
}

impl Name {
    pub fn is_valid(name: &String) -> bool {
        assert!(name.len() <= MAX_NAME_LENGTH, "name's length should be less than 20");
        let name = name.to_lowercase();
        let components = name.split(".").collect::<Vec<&str>>();
        if components.len() != 2 {
            false
        } else if components[1] != ROOT_DOMAIN {
            false
        } else {
            true
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
pub struct Metadata {
    pub name: String,
    pub description: Option<String>,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    resolve_name: LookupMap<String, Name>,
    owned_names: LookupMap<Address, Vec<Name>>,
    denied_names: BTreeSet<String>,
    approved_transfers: BTreeMap<String, Address>,
    administrators: BTreeSet<Address>,
    metadata: Metadata,
}

#[contract]
impl Contract {
    pub fn new() {
        assert_eq!(caller_address(), contract_owner_address(), "Only the owner can call this function");
        Self::assert_if_initialized();

        Self {
            resolve_name: LookupMap::new(STORAGE_RESOLVE_NAME_KEY.to_vec()),
            owned_names: LookupMap::new(STORAGE_OWNED_NAMES_KEY.to_vec()),
            denied_names: BTreeSet::new(),
            approved_transfers: BTreeMap::new(),
            administrators: BTreeSet::new(),
            metadata: Metadata { name: "L1x Name Service".to_string(), description: None },
        }
        .save();
    }

    pub fn register_root_name(name: String, resolver: Option<Address>) {
        let mut contract = Self::load();
        contract.register_root_name_internal(name, resolver);
        contract.save();
    }

    pub fn resolve_name(name: String) -> Address {
        let contract = Self::load();
        contract.resolve_name_internal(name)
    }

    pub fn change_name_resolver(name: String, new_resolver: Address) {
        let mut contract = Self::load();
        contract.change_name_resolver_internal(name, new_resolver);
        contract.save();
    }

    pub fn transfer_name(name: String, new_owner: Address) {
        let mut contract = Self::load();
        contract.transfer_name_internal(name, new_owner);
        contract.save();
    }

    pub fn approve_transfer(name: String, operator: Address) {
        let mut contract = Self::load();
        contract.approve_transfer_internal(name, operator);
        contract.save();
    }

    pub fn remove_all_transfer_approvals(name: String) {
        let mut contract = Self::load();
        contract.remove_all_transfer_approvals_internal(name);
        contract.save();
    }

    pub fn list_transfer_approvals(name: String) -> Vec<(String, Address)> {
        let contract = Self::load();
        contract.list_transfer_approvals_internal(name)
    }

    pub fn owner_of(name: String) -> Address {
        let contract = Self::load();
        contract.owner_of_internal(name)
    }

    pub fn deny_name(name: String) {
        let mut contract = Self::load();
        contract.deny_name_internal(name);
        contract.save();
    }

    pub fn allow_name(name: String) {
        let mut contract = Self::load();
        contract.allow_name_internal(name);
        contract.save();
    }

    pub fn list_denied_names() -> Vec<String> {
        let contract = Self::load();
        contract.list_denied_names_internal()
    }

    pub fn list_owned_names(user: Address) -> Vec<Name> {
        let contract = Self::load();
        contract.list_owned_names_internal(user)
    }

    pub fn add_admin(user: Address) {
        let mut contract = Self::load();
        contract.add_admin_internal(user);
        contract.save();
    }

    pub fn remove_admin(user: Address) {
        let mut contract = Self::load();
        contract.remove_admin_internal(user);
        contract.save();
    }

    pub fn list_admins() -> Vec<Address> {
        let contract = Self::load();
        contract.list_admins_internal()
    }

    pub fn name() -> String {
        let contract = Self::load();
        contract.metadata.name
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

impl Contract {
    fn register_root_name_internal(&mut self, name: String, resolver: Option<Address>) {
        Self::assert_name_is_valid(&name);
        self.assert_name_not_denied(&name);
        assert!(!self.resolve_name.contains_key(&name), "The name is already registered");

        let caller = caller_address();
        let resolver = resolver.unwrap_or(caller.clone());

        let name_record = Name { name: name.clone(), resolver, owner: caller };
        if let Some(names) = self.owned_names.get_mut(&caller) {
            names.push(name_record.clone());
        } else {
            self.owned_names.insert(caller, vec![name_record.clone()]);
        }
        self.resolve_name.insert(name, name_record);
    }

    fn resolve_name_internal(&self, name: String) -> Address {
        self.resolve_name.get(&name).expect("The name is not found").clone().resolver
    }

    fn change_name_resolver_internal(&mut self, name: String, new_resolver: Address) {
        Self::assert_name_is_valid(&name);
        let caller = caller_address();
        let names = self.owned_names.get_mut(&caller).expect("The caller doesn't own any names");
        let name_rec = names.iter_mut().find(|n| n.name == name).expect("The caller doesn't own the specified name");
        name_rec.resolver = new_resolver.clone();
        let name_rec = self.resolve_name.get_mut(&name).expect("Can't find the specified name");
        name_rec.resolver = new_resolver;
    }

    fn transfer_name_internal(&mut self, name: String, new_owner: Address) {
        Self::assert_name_is_valid(&name);
        let caller = caller_address();

        let user = if let Some(operator) = self.approved_transfers.get(&name) {
            if *operator == caller {
                self.approved_transfers.remove(&name);
                let name_rec = self.resolve_name.get_mut(&name).expect("Can't find the specified name");
                name_rec.owner
            } else {
                caller
            }
        } else {
            caller
        };

        let name_rec = {
            // Check whether the user owns this name and modify the record
            let names = self.owned_names.get_mut(&user).expect("The caller doesn't own any names");
            let name_rec_idx =
                names.iter().position(|n| n.name == name).expect("The caller doesn't own the specified name");
            let mut name_rec = names.remove(name_rec_idx);
            name_rec.owner = new_owner;
            name_rec
        };
        {
            let name_rec = self.resolve_name.get_mut(&name).expect("Can't find the specified name");
            name_rec.owner = new_owner;
        }

        if let Some(names) = self.owned_names.get_mut(&new_owner) {
            names.push(name_rec);
        } else {
            self.owned_names.insert(new_owner, vec![name_rec]);
        }
    }

    fn approve_transfer_internal(&mut self, name: String, operator: Address) {
        Self::assert_name_is_valid(&name);
        self.assert_name_not_denied(&name);
        let caller = caller_address();

        // Check whether the caller owns the name
        let names = self.owned_names.get(&caller).expect("The caller doesn't own any names");
        names.iter().find(|n| n.name == name).expect("The caller doesn't own the specified name");

        self.approved_transfers.insert(name, operator);
    }

    fn remove_all_transfer_approvals_internal(&mut self, name: String) {
        Self::assert_name_is_valid(&name);
        let caller = caller_address();

        self.assert_owner_of_name(&name, &caller);

        self.approved_transfers.remove(&name).expect("Can't find a transfer approval for the specified name");
    }

    fn list_transfer_approvals_internal(&self, name: String) -> Vec<(String, Address)> {
        if let Some(operator) = self.approved_transfers.get(&name) {
            vec![(name, operator.clone())]
        } else {
            vec![]
        }
    }

    fn deny_name_internal(&mut self, name: String) {
        Self::assert_name_is_valid(&name);
        self.assert_name_not_denied(&name);
        self.assert_owner_or_admin();
        self.denied_names.insert(name.clone());
        // Unregister the denied name
        let name_rec = self.resolve_name.remove(name.clone()).expect("Can't find the name");
        let names = self.owned_names.get_mut(&name_rec.owner).expect("Can't find the owner");
        let name_rec_idx = names.iter().position(|n| n.name == name).expect("Can't find the name");
        names.remove(name_rec_idx);
        self.approved_transfers.remove(&name);
    }

    fn allow_name_internal(&mut self, name: String) {
        Self::assert_name_is_valid(&name);
        self.assert_owner_or_admin();

        self.denied_names.remove(&name);
    }

    fn list_denied_names_internal(&self) -> Vec<String> {
        self.denied_names.iter().cloned().collect()
    }

    fn list_owned_names_internal(&self, user: Address) -> Vec<Name> {
        let binding = Vec::new();
        let names = self.owned_names.get(&user).unwrap_or(&binding);
        names.clone()
    }

    fn owner_of_internal(&self, name: String) -> Address {
        let name_rec = self.resolve_name.get(&name).expect("Can't find the specified name");
        name_rec.owner
    }

    fn add_admin_internal(&mut self, user: Address) {
        Self::assert_owner();

        assert!(self.administrators.insert(user), "The provided user is already an administrator")
    }

    fn remove_admin_internal(&mut self, user: Address) {
        Self::assert_owner();

        self.administrators.remove(&user);
    }

    fn list_admins_internal(&self) -> Vec<Address> {
        self.administrators.iter().cloned().collect()
    }

    fn assert_owner_of_name(&self, name: &String, user: &Address) {
        let owner = self.resolve_name.get(name).expect("Can't find the specified name").owner;
        assert_eq!(owner, *user, "The user doesn't own the name");
    }

    fn assert_name_is_valid(name: &String) {
        assert!(Name::is_valid(&name), "The name must be specified in the following format: NAME.l1x");
    }

    fn assert_name_not_denied(&self, name: &String) {
        assert!(!self.denied_names.contains(name), "The name is denied");
    }

    fn assert_owner_or_admin(&self) {
        let caller = caller_address();
        assert!(
            caller == contract_owner_address() || self.administrators.contains(&caller),
            "This function can be called only by the owner or an administrator"
        );
    }

    fn assert_owner() {
        assert_eq!(caller_address(), contract_owner_address(), "This function can be called only by the owner");
    }
}