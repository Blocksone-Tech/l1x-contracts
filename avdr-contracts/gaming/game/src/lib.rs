use std::collections::{BTreeMap, BTreeSet};

use borsh::{BorshDeserialize, BorshSerialize};
use l1x_sdk::store::LookupMap;
use l1x_sdk::types::Address;
use l1x_sdk::{caller_address, contract, contract_instance_address, contract_owner_address};
use serde::Serialize;
use types::score_board_interface::ScoreBoardContract;

const STORAGE_CONTRACT_KEY: &[u8] = b"STATE";
const STORAGE_BOARDS_KEY: &[u8] = b"BOARDS";

#[derive(BorshSerialize, BorshDeserialize, Serialize, Clone)]
pub struct User {
    pub is_ready: bool,
}

impl User {
    pub fn new() -> Self {
        Self { is_ready: false }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize, Clone)]
pub struct Lobby {
    pub board: Address,
    pub locked: bool,
    pub users: BTreeMap<Address, User>,
}

impl Lobby {
    pub fn new(board: Address, users: Vec<Address>) -> Self {
        let users = users.iter().map(|a| (a.clone(), User::new())).collect();
        Self { board, locked: false, users }
    }

    pub fn join(&mut self, user: Address) {
        assert!(!self.locked, "Lobby is locked");
        assert!(self.users.insert(user, User::new()).is_none(), "The user is aleady joined");
    }

    pub fn leave(&mut self, user: &Address) {
        assert!(self.users.remove(user).is_none(), "Can't find the user");
    }

    pub fn set_ready(&mut self, user: &Address, status: bool) {
        assert!(!self.locked, "Lobby is locked");
        self.users.get_mut(user).expect("Can't find the user").is_ready = status;
    }

    pub fn is_ready(&self, user: &Address) -> bool {
        self.users.get(user).expect("Can't find the user").is_ready
    }

    pub fn ready_users(&self) -> Vec<Address> {
        self.users.iter().filter(|(_, user_status)| user_status.is_ready).map(|(user, _)| user.clone()).collect()
    }

    pub fn lock(&mut self) {
        self.locked = true;
    }
}

#[derive(BorshSerialize, BorshDeserialize, Serialize)]
pub struct Metadata {
    pub name: String,
    pub description: Option<String>,
    pub game_id: String,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    metadata: Metadata,
    // session_id -> Score Board contract
    registered_boards: LookupMap<String, Lobby>,
    administrators: BTreeSet<Address>,
}

#[contract]
impl Contract {
    pub fn new(name: String, description: Option<String>, game_id: String) {
        assert_eq!(caller_address(), contract_owner_address(), "Only the owner can call this function");
        Self::assert_if_initialized();

        Self {
            metadata: Metadata { name, description, game_id },
            registered_boards: LookupMap::new(STORAGE_BOARDS_KEY.to_vec()),
            administrators: BTreeSet::new(),
        }
        .save();
    }

    pub fn create_lobby(session_id: String, board: Address) {
        let mut contract = Self::load();
        contract.create_lobby_internal(session_id, board);
        contract.save();
    }

    pub fn join_lobby(session_id: String) {
        let mut contract = Self::load();
        contract.join_lobby_internal(session_id);
        contract.save();
    }

    pub fn leave_lobby(session_id: String) {
        let mut contract = Self::load();
        contract.leave_lobby_internal(session_id);
        contract.save();
    }

    pub fn set_user_ready_status(session_id: String, status: bool) {
        let mut contract = Self::load();
        contract.set_user_ready_status_internal(session_id, status);
        contract.save();
    }

    pub fn start_game(session_id: String) {
        let mut contract = Self::load();
        contract.start_game_internal(session_id);
        contract.save();
    }

    pub fn end_game(session_id: String) {
        let mut contract = Self::load();
        contract.end_game_internal(session_id);
        contract.save();
    }

    pub fn get_board_by_session(session_id: String) -> Address {
        let contract = Self::load();
        contract.get_board_by_session_internal(session_id)
    }

    pub fn get_lobby_by_session(session_id: String) -> Lobby {
        let contract = Self::load();
        contract.get_lobby_by_session_internal(session_id)
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
    fn create_lobby_internal(&mut self, session_id: String, board: Address) {
        assert!(!self.registered_boards.contains_key(&session_id), "This session already has a registered board");

        let user = caller_address();

        Self::assert_rules(&user);

        let board_contract = ScoreBoardContract::new(board);
        let board_metadata = board_contract.metadata();
        assert_eq!(
            board_metadata.game_id, self.metadata.game_id,
            "Game ID in Score board must be equal to the one in the current contract"
        );
        assert_eq!(
            board_metadata.session_id, session_id,
            "Session ID in Score board must equal to the current Session ID"
        );
        assert_eq!(
            board_contract.administrator(),
            contract_instance_address(),
            "The current contract must be an administrator in the Score board"
        );
        assert!(board_contract.users().is_empty(), "Score board must not have any users");

        let lobby = Lobby::new(board, vec![user]);

        self.registered_boards.insert(session_id, lobby);
    }

    fn join_lobby_internal(&mut self, session_id: String) {
        let lobby = self.registered_boards.get_mut(&session_id).expect("Can't find the session");

        let user = caller_address();

        Self::assert_rules(&user);

        lobby.join(user);
    }

    fn leave_lobby_internal(&mut self, session_id: String) {
        let lobby = self.registered_boards.get_mut(&session_id).expect("Can't find the session");
        let user = caller_address();
        lobby.leave(&user);
    }

    fn set_user_ready_status_internal(&mut self, session_id: String, status: bool) {
        let lobby = self.registered_boards.get_mut(&session_id).expect("Can't find the session");
        lobby.set_ready(&caller_address(), status);
    }

    fn start_game_internal(&mut self, session_id: String) {
        self.assert_owner_or_admin();

        let lobby = self.registered_boards.get_mut(&session_id).expect("Can't find the session");
        let board = lobby.board;

        Self::assert_lobby_not_locked(&lobby);

        let ready_users = lobby.ready_users();
        assert!(!ready_users.is_empty(), "There are not ready users");

        let mut board_contract = ScoreBoardContract::new(board);
        board_contract.start_game(ready_users);

        lobby.lock();
    }

    fn end_game_internal(&mut self, session_id: String) {
        self.assert_owner_or_admin();

        let lobby = self.registered_boards.get_mut(&session_id).expect("Can't find the session");
        let board = lobby.board;

        let mut board_contract = ScoreBoardContract::new(board);
        board_contract.end_game();
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

    fn get_board_by_session_internal(&self, session_id: String) -> Address {
        self.registered_boards.get(&session_id).expect("Can't find the session").board.clone()
    }

    fn get_lobby_by_session_internal(&self, session_id: String) -> Lobby {
        self.registered_boards.get(&session_id).expect("Can't find the session").clone()
    }

    fn assert_lobby_not_locked(lobby: &Lobby) {
        assert!(!lobby.locked, "The lobby is locked");
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

    fn assert_rules(_user: &Address) {}
}