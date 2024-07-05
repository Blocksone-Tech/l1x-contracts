use borsh::{BorshDeserialize, BorshSerialize};
use l1x_sdk::store::LookupMap;
use l1x_sdk::types::Address;
use l1x_sdk::{caller_address, contract, contract_owner_address};
use serde::Serialize;
use types::access_control_interface::{Permission, ReqIdx};
use types::data_storage_interface::DataStorageContract;
use types::{Cid, PublicKey};

use std::collections::{BTreeMap, BTreeSet};

const STORAGE_CONTRACT_KEY: &[u8] = b"STATE";
const STORAGE_PERMISSIONS_KEY: &[u8] = b"PERMISSIONS";
const STORAGE_SHARED_KEY: &[u8] = b"SHARED";
const STORAGE_REQ_STATUS_KEY: &[u8] = b"REQ_STATUS";
const STORAGE_DENIED_PUB_KEY: &[u8] = b"DENIED_PUB_KEYS";
const STORAGE_DENIED_CIDS_KEY: &[u8] = b"DENIED_CIDS";

#[derive(BorshSerialize, BorshDeserialize, Serialize, Clone, Debug)]
pub struct ShareRequest {
    pub user: Address,
    pub pub_key: PublicKey,
    pub cid: Cid,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Clone, Debug)]
pub struct PermissionRequest {
    pub user: Address,
    pub cid: Cid,
    pub perm: Permission,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Clone, Debug)]
pub enum Request {
    Permission(PermissionRequest),
    Share(ShareRequest),
}

impl Request {
    pub fn cid(&self) -> &Cid {
        match self {
            Request::Permission(PermissionRequest { cid, .. }) | Request::Share(ShareRequest { cid, .. }) => cid,
        }
    }

    pub fn user(&self) -> &Address {
        match self {
            Request::Permission(PermissionRequest { user, .. }) | Request::Share(ShareRequest { user, .. }) => user,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Clone, Debug)]
pub struct PendingRequest {
    pub req_idx: ReqIdx,
    pub req: Request,
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Clone, Debug)]
pub enum RequestStatus {
    Pending,
    Rejected,
    Approved,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    pending_requests: BTreeMap<ReqIdx, Request>,
    approved_permissions: LookupMap<Cid, BTreeMap<Address, BTreeSet<Permission>>>,
    approved_shares: LookupMap<Cid, BTreeSet<Cid>>,
    request_status: LookupMap<ReqIdx, RequestStatus>,

    denied_pub_keys: LookupMap<PublicKey, ()>,
    denied_cids: LookupMap<Cid, ()>,

    requests_counter: u64,
}

#[contract]
impl Contract {
    pub fn new() {
        assert_eq!(caller_address(), contract_owner_address(), "Only the owner can call this function");
        Self::assert_if_initialized();

        Self {
            pending_requests: BTreeMap::new(),
            approved_permissions: LookupMap::new(STORAGE_PERMISSIONS_KEY.to_vec()),
            approved_shares: LookupMap::new(STORAGE_SHARED_KEY.to_vec()),
            request_status: LookupMap::new(STORAGE_REQ_STATUS_KEY.to_vec()),
            denied_pub_keys: LookupMap::new(STORAGE_DENIED_PUB_KEY.to_vec()),
            denied_cids: LookupMap::new(STORAGE_DENIED_CIDS_KEY.to_vec()),
            requests_counter: 0,
        }
        .save();
    }

    pub fn permissions(user: Address, cid: Cid) -> Vec<Permission> {
        let contract = Self::load();
        contract.permissions_internal(user, cid)
    }

    pub fn has_perm(user: Address, cid: Cid, perm: Permission) -> bool {
        let contract = Self::load();
        contract.has_perm_internal(user, cid, perm)
    }

    pub fn approved_shares(cid: Cid) -> Vec<Cid> {
        let contract = Self::load();
        contract.approved_shares_internal(cid)
    }

    pub fn has_approved_share(cid: Cid, from_cid: Cid) -> bool {
        let contract = Self::load();
        contract.has_approved_share_internal(cid, from_cid)
    }

    pub fn status_of_request(req_idx: ReqIdx) -> RequestStatus {
        let contract = Self::load();
        contract.status_of_request_internal(req_idx)
    }

    pub fn request_perm(cid: Cid, perm: Permission) -> ReqIdx {
        let mut contract = Self::load();
        let req = Request::Permission(PermissionRequest { user: caller_address(), cid, perm });
        let req_idx = contract.request_perm_internal(req);
        contract.save();

        req_idx
    }

    pub fn approve_perm(req_idx: ReqIdx) {
        let mut contract = Self::load();
        contract.approve_perm_internal(req_idx, None);
        contract.save();
    }

    pub fn request_share(cid: Cid, pub_key: PublicKey) -> ReqIdx {
        let mut contract = Self::load();
        let req = Request::Share(ShareRequest { user: caller_address(), cid, pub_key });
        let req_idx = contract.request_perm_internal(req);
        contract.save();

        req_idx
    }

    pub fn approve_share(req_idx: ReqIdx, shared_cid: Cid) {
        let mut contract = Self::load();
        contract.approve_perm_internal(req_idx, Some(shared_cid));
        contract.save();
    }

    pub fn reject_request(req_idx: ReqIdx) {
        let mut contract = Self::load();
        contract.reject_perm_internal(req_idx);
        contract.save();
    }

    pub fn revoke_perm(cid: Cid, user: Address, perm: Permission) {
        let mut contract = Self::load();
        contract.revoke_perm_internal(cid, user, perm);
        contract.save();
    }

    pub fn pending_requests_by_cid(cid: Cid) -> Vec<PendingRequest> {
        let contract = Self::load();

        contract
            .pending_requests
            .into_iter()
            .filter(|(_, req)| req.cid().clone() == cid)
            .map(|(req_idx, req)| PendingRequest { req_idx, req })
            .collect::<_>()
    }

    pub fn pending_requests_by_user(user: Address) -> Vec<PendingRequest> {
        let contract = Self::load();

        contract
            .pending_requests
            .into_iter()
            .filter(|(_, req)| req.user().clone() == user)
            .map(|(req_idx, req)| PendingRequest { req_idx, req })
            .collect::<_>()
    }

    pub fn deny_pubkey(pub_key: PublicKey) {
        let mut contract = Self::load();

        contract.deny_pubkey_internal(pub_key);
        contract.save();
    }

    pub fn pubkey_is_denied(pub_key: PublicKey) -> bool {
        let contract = Self::load();

        contract.denied_pub_keys.contains_key(&pub_key)
    }

    pub fn deny_cid(cid: Cid) {
        let mut contract = Self::load();

        contract.deny_cid_internal(cid);
        contract.save();
    }

    pub fn cid_is_denied(cid: Cid) -> bool {
        let contract = Self::load();

        contract.denied_cids.contains_key(&cid)
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
    fn assert_if_denied(&self, req: &Request) {
        assert!(!self.denied_cids.contains_key(req.cid()), "The cid is denied");
        if let Request::Share(req) = req {
            assert!(!self.denied_pub_keys.contains_key(&req.pub_key), "The public key is denied");
        }
    }

    fn permissions_internal(&self, user: Address, cid: Cid) -> Vec<Permission> {
        let owner = l1x_sdk::contract_owner_address_of(cid.0);

        if user == owner {
            vec![Permission::Write, Permission::Remove, Permission::Approve, Permission::Reject, Permission::Revoke]
        } else {
            let users = self.approved_permissions.get(&cid).expect("The provided cid is not found");
            let perms = users.get(&user).expect("The provided user is not found");

            perms.iter().cloned().collect::<_>()
        }
    }

    fn has_perm_internal(&self, user: Address, cid: Cid, perm: Permission) -> bool {
        self.permissions_internal(user, cid).into_iter().find(|p| perm == *p).is_some()
    }

    fn approved_shares_internal(&self, cid: Cid) -> Vec<Cid> {
        let shared_cids = self.approved_shares.get(&cid).expect("The provided cid is not found");
        shared_cids.iter().cloned().collect::<_>()
    }

    fn has_approved_share_internal(&self, cid: Cid, from_cid: Cid) -> bool {
        let shared_cids = self.approved_shares.get(&from_cid).expect("The provided cid is not found");
        shared_cids.iter().any(|c| *c == cid)
    }

    fn status_of_request_internal(&self, req_idx: ReqIdx) -> RequestStatus {
        self.request_status.get(&req_idx).expect("The provided request is not found").clone()
    }

    fn request_perm_internal(&mut self, req: Request) -> ReqIdx {
        self.assert_if_denied(&req);

        self.requests_counter += 1;
        let req_idx = ReqIdx(self.requests_counter.into());

        self.pending_requests.insert(req_idx.clone(), req);
        self.request_status.insert(req_idx.clone(), RequestStatus::Pending);

        req_idx
    }

    fn approve_perm_internal(&mut self, req_idx: ReqIdx, shared_cid: Option<Cid>) {
        let req = self.pending_requests.get(&req_idx).expect("The provided request is not found");

        self.assert_if_denied(&req);
        assert!(
            self.has_perm_internal(caller_address(), req.cid().clone(), Permission::Approve),
            "The caller doesn't have 'approve' permission"
        );

        match req {
            Request::Permission(req) => {
                assert!(shared_cid.is_none(), "WARNING: Shared CID is ignored in this request.");

                if let Some(users) = self.approved_permissions.get_mut(&req.cid) {
                    if let Some(perms) = users.get_mut(&req.user) {
                        perms.insert(req.perm.clone());
                    } else {
                        let mut perms = BTreeSet::new();
                        perms.insert(req.perm.clone());
                        users.insert(req.user, perms);
                    }
                } else {
                    let mut perms = BTreeSet::new();
                    perms.insert(req.perm.clone());
                    let mut users = BTreeMap::new();

                    users.insert(req.user, perms);
                    self.approved_permissions.insert(req.cid.clone(), users);
                }
            }
            Request::Share(req) => {
                let shared_cid = shared_cid.expect("Shared CID is required");
                assert!(!self.approved_shares.contains_key(&shared_cid), "The provided Shared CID is already approved");

                let data_storage = DataStorageContract::new(shared_cid.0);
                assert_eq!(data_storage.public_key(), req.pub_key, "The public keys don't match");

                if let Some(shared_cids) = self.approved_shares.get_mut(&req.cid) {
                    shared_cids.insert(shared_cid);
                } else {
                    let mut shared_cids = BTreeSet::new();
                    shared_cids.insert(shared_cid);
                    self.approved_shares.insert(req.cid.clone(), shared_cids);
                }
            }
        }

        self.pending_requests.remove(&req_idx);
        self.request_status.insert(req_idx, RequestStatus::Approved);
    }

    fn reject_perm_internal(&mut self, req_idx: ReqIdx) {
        // Don't check whether pub_key is denied because then we will not be to able to remove the pending request
        // self.assert_if_denied(&pub_key);
        let req = self.pending_requests.get(&req_idx).expect("The provided req_idx is not found");
        assert!(
            self.has_perm_internal(caller_address(), req.cid().clone(), Permission::Reject),
            "The caller doesn't have 'reject' permission"
        );

        let _old_req = self.pending_requests.remove(&req_idx).expect("The provided request is not found");
        self.request_status.insert(req_idx, RequestStatus::Rejected);
    }

    fn revoke_perm_internal(&mut self, cid: Cid, user: Address, perm: Permission) {
        // Don't check whether pub_key is denied because then we will not be to able to remove the pending request
        // self.assert_if_denied(&pub_key);

        assert!(
            self.has_perm_internal(caller_address(), cid.clone(), Permission::Revoke),
            "The caller doesn't have 'reject' permission"
        );

        let users = self.approved_permissions.get_mut(&cid).expect("The provided cid is not found");
        let perms = users.get_mut(&user).expect("The provided user is not found");

        assert!(perms.remove(&perm), "The user didn't have such permissions for the provided cid");

        if perms.is_empty() {
            users.remove(&user);
        }

        if users.is_empty() {
            self.approved_permissions.remove(cid);
        }
    }

    fn deny_pubkey_internal(&mut self, pub_key: PublicKey) {
        assert_eq!(contract_owner_address(), caller_address(), "Only the contract owner can call this function");

        self.denied_pub_keys.insert(pub_key, ());
    }

    fn deny_cid_internal(&mut self, cid: Cid) {
        assert_eq!(contract_owner_address(), caller_address(), "Only the contract owner can call this function");

        self.denied_cids.insert(cid, ());
    }
}