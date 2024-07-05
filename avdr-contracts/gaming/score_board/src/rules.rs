use borsh::{BorshDeserialize, BorshSerialize};
use l1x_sdk::types::Address;

pub enum ActionType {
    Default,
}

pub struct RuleAction {
    pub user: Address,
    pub action: ActionType,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Rules;

impl Rules {
    pub fn process_rules(&self, _action: &RuleAction) -> u64 {
        1
    }
}