use std::collections::BTreeMap;

use borsh::{BorshDeserialize, BorshSerialize};
use l1x_sdk::types::{Address, U64};
use serde::Serialize;

use crate::rules::{RuleAction, Rules};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Clone, Copy)]
pub struct Scores {
    user: Address,
    scores: U64,
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Board {
    rules: Rules,
    scores: BTreeMap<Address, Scores>,
}

impl Board {
    pub fn new(users: Vec<Address>, rules: Rules) -> Self {
        let scores_iter = users.iter().cloned().map(|user| (user, Scores { user, scores: 0.into() }));
        Self { rules, scores: BTreeMap::from_iter(scores_iter) }
    }

    pub fn update(&mut self, action: &RuleAction) {
        let scores = self.rules.process_rules(action);
        self.scores.get_mut(&action.user).expect("Can't find the user").scores.0 += scores;
    }

    pub fn leader_board(&self) -> Vec<Scores> {
        let mut v = self.scores.values().cloned().collect::<Vec<_>>();
        v.sort_by(|a, b| a.scores.0.cmp(&b.scores.0));
        v
    }
}