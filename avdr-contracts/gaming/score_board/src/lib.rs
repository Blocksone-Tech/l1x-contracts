mod board;
mod rules;

use board::{Board, Scores};
use borsh::{BorshDeserialize, BorshSerialize};
use l1x_sdk::store::Vector;
use l1x_sdk::types::{Address, TimeStamp};
use l1x_sdk::{block_timestamp, caller_address, contract, contract_owner_address};
use rules::RuleAction;
use serde::Serialize;
use types::{EventData, ScoreBoardMetadata};

use std::collections::BTreeSet;

const STORAGE_CONTRACT_KEY: &[u8] = b"STATE";
const STORAGE_EVENTS_KEY: &[u8] = b"EVENTS";

#[derive(BorshSerialize, BorshDeserialize, Serialize, Clone, Debug)]
pub struct Event {
    timestamp: TimeStamp,
    reporter: Address,
    data: EventData,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    metadata: ScoreBoardMetadata,
    users: BTreeSet<Address>,
    events: Vector<Event>,
    board: Option<Board>,
    administrator: Address,
    locked: bool,
    game_is_started: bool,
}

#[contract]
impl Contract {
    pub fn new(name: String, description: Option<String>, game_id: String, session_id: String, administrator: Address) {
        assert_eq!(caller_address(), contract_owner_address(), "Only the owner can call this function");
        Self::assert_not_initialized();

        Self {
            metadata: ScoreBoardMetadata { name, description, game_id, session_id },
            users: BTreeSet::new(),
            events: Vector::new(STORAGE_EVENTS_KEY.to_vec()),
            board: None,
            administrator,
            locked: false,
            game_is_started: false,
        }
        .save();
    }

    pub fn start_game(users: Vec<Address>) {
        let mut contract = Self::load();
        contract.start_game_internal(users);
        contract.save();
    }

    pub fn store_event(event: EventData) {
        let mut contract = Self::load();
        contract.store_event_internal(event);
        contract.save();
    }

    pub fn end_game() {
        let mut contract = Self::load();
        contract.end_game_internal();
        contract.save();
    }

    pub fn board() -> Vec<Scores> {
        let contract = Self::load();
        contract.board_internal()
    }

    pub fn events() -> Vec<Event> {
        let contract = Self::load();
        contract.events_internal()
    }

    pub fn name() -> String {
        let contract = Self::load();
        contract.metadata.name
    }

    pub fn metadata() -> ScoreBoardMetadata {
        let contract = Self::load();
        contract.metadata
    }

    pub fn administrator() -> Address {
        let contract = Self::load();
        contract.administrator
    }

    pub fn users() -> Vec<Address> {
        let contract = Self::load();
        contract.users_internal()
    }

    fn assert_not_initialized() {
        assert!(l1x_sdk::storage_read(STORAGE_CONTRACT_KEY).is_none(), "The contract is already initialized");
    }

    fn assert_not_locked(&self) {
        assert!(!self.locked, "The contract has been locked");
    }

    fn assert_game_not_started(&self) {
        assert!(!self.game_is_started, "The game is already started");
    }

    fn assert_user(&self, user: &Address) {
        assert!(self.users.contains(user), "Can't find a user");
    }

    fn assert_administrator(&self, user: &Address) {
        assert_eq!(self.administrator, *user)
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
    fn start_game_internal(&mut self, users: Vec<Address>) {
        self.assert_not_locked();
        self.assert_game_not_started();
        self.assert_administrator(&caller_address());

        users.iter().for_each(|u| {
            self.users.insert(u.clone());
        });
        self.board = Some(Board::new(users, rules::Rules {}));

        self.game_is_started = true;
    }

    fn store_event_internal(&mut self, event: EventData) {
        self.assert_not_locked();

        let user = caller_address();
        self.assert_user(&user);

        self.events.push(Event { timestamp: block_timestamp(), reporter: user, data: event });
        // TODO: Convert Event to ActionType
        let action = rules::ActionType::Default;
        self.board.as_mut().expect("Board isn't initialized").update(&RuleAction { user, action });
    }

    fn end_game_internal(&mut self) {
        self.assert_not_locked();
        self.assert_administrator(&caller_address());

        self.locked = true;
    }

    fn board_internal(&self) -> Vec<Scores> {
        self.board.as_ref().expect("Board isn't initialized").leader_board()
    }

    fn events_internal(&self) -> Vec<Event> {
        let len = self.events.len();
        let mut events = Vec::with_capacity(len as _);
        for i in 0..len {
            events.push(self.events[i].clone());
        }

        events
    }

    fn users_internal(&self) -> Vec<Address> {
        self.users.iter().cloned().collect()
    }
}