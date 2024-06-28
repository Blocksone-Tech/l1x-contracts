mod base64_data;

use std::collections::BTreeMap;

use base64_data::Base64Data;
use borsh::{BorshDeserialize, BorshSerialize};
use l1x_sdk::{
    caller_address, contract, contract_owner_address, emit_event_experimental,
    store::LookupMap,
    types::{Address, U128},
};
use serde::{Deserialize, Serialize};

/// Key for the storage of the contract data.
const STORAGE_CONTRACT_KEY: &[u8] = b"state";

/// Key for the storage of the balance data.
const STORAGE_BALANCE_OF_KEY: &[u8] = b"balances";

/// Key for the storage of the approval status data.
const STORAGE_IS_APPROVED_FOR_ALL_KEY: &[u8] = b"approved-all";

#[derive(BorshSerialize, BorshDeserialize, Deserialize)]
pub struct NFTMetadata {
    name: String,
    icon: Option<String>,
    uri: String,
}

#[derive(Debug, BorshSerialize, Serialize, Clone)]
enum Erc1155Event {
    TransferSingle {
        operator: Address,
        from: Address,
        to: Address,
        id: U128,
        value: U128,
    },
    TransferBatch {
        operator: Address,
        from: Address,
        to: Address,
        ids: Vec<U128>,
        values: Vec<U128>,
    },
    ApprovalForAll {
        owner: Address,
        operator: Address,
        approved: bool,
    },
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct NftContract {
    metadata: NFTMetadata,
    balance_of: LookupMap<Address, BTreeMap<u128, u128>>,
    is_approved_for_all: LookupMap<Address, BTreeMap<Address, bool>>,
}

#[contract]
impl NftContract {
    pub fn new(metadata: NFTMetadata) {
        Self::assert_caller_is_owner();
        Self::assert_not_initialized();

        let mut contract = Self {
            metadata,
            balance_of: LookupMap::new(STORAGE_BALANCE_OF_KEY.to_vec()),
            is_approved_for_all: LookupMap::new(STORAGE_IS_APPROVED_FOR_ALL_KEY.to_vec()),
        };
        contract.save();
    }

    pub fn nft_name() -> String {
        let contract = Self::load();
        contract.metadata.name
    }

    pub fn nft_uri(id: U128) -> String {
        let contract = Self::load();
        contract.metadata.uri + &id.0.to_string() + ".json"
    }

    pub fn nft_icon() -> Option<String> {
        let contract = Self::load();
        contract.metadata.icon
    }

    pub fn nft_mint_id(to: Address, id: U128, amount: U128) {
        // load the contract storage state
        let mut contract = Self::load();

        // Call the internal implementation
        contract.mint_id_to(to, id.into(), amount.into());

        // Save the contract state
        contract.save();
    }

    pub fn nft_burn(account: Address, id: U128, amount: U128) {
        // load the contract storage state
        let mut contract = Self::load();

        // Call the internal implementation
        contract.burn(account, id.into(), amount.into());

        // Save the contract state
        contract.save();
    }

    pub fn nft_set_approval_for_all(operator: Address, approved: bool) {
        // load the contract storage state
        let mut contract = Self::load();

        // Call the internal implementation
        contract.set_approval_for_all(operator, approved);

        // Save the contract state
        contract.save();
    }

    pub fn nft_balance_of(owner: Address, id: U128) -> U128 {
        // load the contract storage state
        let contract = Self::load();

        // Call the internal implementation
        contract.balance_of(owner, id.into()).into()
    }

    pub fn nft_owned_tokens(owner: Address) -> Vec<(U128, U128)> {
        // load the contract storage state
        let contract = Self::load();

        // Call the internal implementation
        contract.owned_tokens(owner)
    }

    pub fn nft_safe_transfer_from(
        from: Address,
        to: Address,
        id: U128,
        amount: U128,
        #[allow(unused_variables)] calldata: Base64Data,
    ) {
        // load the contract storage state
        let mut contract = Self::load();

        // Call the internal implementation
        contract.safe_transfer_from(from, to, id.into(), amount.into());

        // Save the contract state
        contract.save();
    }

    pub fn nft_safe_batch_transfer_from(
        from: Address,
        to: Address,
        ids: Vec<U128>,
        values: Vec<U128>,
        #[allow(unused_variables)] calldata: Base64Data,
    ) {
        // load the contract storage state
        let mut contract = Self::load();

        // Call the internal implementation
        contract.safe_batch_transfer_from(from, to, ids, values);

        // Save the contract state
        contract.save();
    }

    pub fn nft_balance_of_batch(owners: Vec<Address>, ids: Vec<U128>) -> Vec<U128> {
        // load the contract storage state
        let contract = Self::load();
        contract.balance_of_batch(owners, ids)
    }

    pub fn nft_is_approved_for_all(owner: Address, operator: Address) -> bool {
        let contract = Self::load();
        contract.is_approved_for_all(&owner, &operator)
    }
}

impl NftContract {
    fn internal_balance_update(
        &mut self,
        account: Address,
        id: u128,
        amount: u128,
        decrease: bool,
    ) {
        // Update the balances
        if let Some(tokens) = self.balance_of.get_mut(&account) {
            let mut found_zero_balance = false;
            if let Some(balance) = tokens.get_mut(&id) {
                let new_balance = if decrease {
                    balance.checked_sub(amount).expect("Not enough fund")
                } else {
                    balance.checked_add(amount).expect("Integer overflow")
                };

                *balance = new_balance;

                if new_balance == 0 {
                    found_zero_balance = true;
                }
            } else {
                // No tokens case
                assert!(!decrease, "No fund");
                tokens.insert(id, amount);
            }

            // Remove useless records
            if found_zero_balance {
                tokens.remove(&id);
            }
        } else {
            // No any tokens case
            assert!(!decrease, "No fund");
            let mut new_map = BTreeMap::new();
            new_map.insert(id, amount);
            self.balance_of.insert(account, new_map);
        };
    }

    fn mint_id_to(&mut self, to: Address, id: u128, amount: u128) {
        Self::assert_caller_is_owner();

        self.internal_balance_update(to, id, amount, false);

        let event = Erc1155Event::TransferSingle {
            operator: caller_address(),
            from: Address::from([0; 20]),
            to,
            id: id.into(),
            value: amount.into(),
        };

        l1x_sdk::msg(&format!(
            "{}",
            serde_json::to_string(&event).unwrap_or_default()
        ));
        // Emit the Token minted event
        emit_event_experimental(event);
    }

    fn burn(&mut self, account: Address, id: u128, amount: u128) {
        Self::assert_caller_is_owner();

        self.internal_balance_update(account, id, amount, true);

        let event = Erc1155Event::TransferSingle {
            operator: caller_address(),
            from: account,
            to: Address::from([0; 20]),
            id: id.into(),
            value: amount.into(),
        };

        // Emit the Token burned event
        emit_event_experimental(event.clone());

        l1x_sdk::msg(&format!(
            "{}",
            serde_json::to_string(&event).unwrap_or_default()
        ));
    }

    fn set_approval_for_all(&mut self, operator: Address, approved: bool) {
        // Get the caller Address
        let caller_id = l1x_sdk::caller_address();

        // Modify the state of `is_approved_for_all`
        if let Some(approved_map) = self.is_approved_for_all.get_mut(&caller_id) {
            // Borrow the value as mutable using `get_mut` and then insert the new key-value pair
            approved_map.insert(operator.clone(), approved);
        } else {
            // If the entry doesn't exist, create a new map, insert the pair, and then insert the new map into `is_approved_for_all`
            let mut new_approved_map = BTreeMap::new();
            new_approved_map.insert(operator.clone(), approved);
            self.is_approved_for_all
                .insert(caller_id.clone(), new_approved_map);
        }

        let event = Erc1155Event::ApprovalForAll {
            owner: caller_id,
            operator,
            approved,
        };
        // Emit the approval for All done event
        emit_event_experimental(event.clone());

        l1x_sdk::msg(&format!(
            "{}",
            serde_json::to_string(&event).unwrap_or_default()
        ));
    }

    fn safe_transfer_from(&mut self, from: Address, to: Address, id: u128, amount: u128) {
        Self::assert_zero_address(&to);
        self.assert_caller_is_approved(&from);

        self.internal_balance_update(from, id, amount, true);
        self.internal_balance_update(to, id, amount, false);

        let event = Erc1155Event::TransferSingle {
            operator: l1x_sdk::caller_address(),
            from,
            to,
            id: id.into(),
            value: amount.into(),
        };
        // Emit transfer done event
        emit_event_experimental(event.clone());

        l1x_sdk::msg(&format!(
            "{}",
            serde_json::to_string(&event).unwrap_or_default()
        ));
    }

    fn safe_batch_transfer_from(
        &mut self,
        from: Address,
        to: Address,
        ids: Vec<U128>,
        values: Vec<U128>,
    ) {
        assert_eq!(
            ids.len(),
            values.len(),
            "Ids and values have must be same length"
        );
        Self::assert_zero_address(&to);
        self.assert_caller_is_approved(&from);

        if !self.balance_of.contains_key(&to) {
            let new_map = BTreeMap::new();
            self.balance_of.insert(to, new_map);
        }
        let mut cloned_tokens_to = self
            .balance_of
            .get(&to)
            .cloned()
            .expect("Can't get just created map");

        let tokens_from = self.balance_of.get_mut(&from).expect("No enough fund");
        ids.iter().zip(values.iter()).for_each(|(id, amount)| {
            let id: u128 = (*id).into();
            let amount: u128 = (*amount).into();

            let balance_from = tokens_from
                .get_mut(&id)
                .expect(&format!("No enough fund: Id {}", id));
            *balance_from = balance_from
                .checked_sub(amount)
                .expect(&format!("No enough fund: Id {}", id));

            let mut balance_to = *cloned_tokens_to.get(&id).unwrap_or(&0u128);
            balance_to = balance_to.checked_add(amount).expect("Integer overflow");
            cloned_tokens_to.insert(id, balance_to);
        });

        self.balance_of.insert(to, cloned_tokens_to);

        let event = Erc1155Event::TransferBatch {
            operator: l1x_sdk::caller_address(),
            from,
            to,
            ids,
            values,
        };
        // Emit transfer done event
        emit_event_experimental(event.clone());

        l1x_sdk::msg(&format!(
            "{}",
            serde_json::to_string(&event).unwrap_or_default()
        ));
    }

    fn balance_of(&self, owner: Address, id: u128) -> u128 {
        *self
            .balance_of
            .get(&owner)
            .unwrap_or(&BTreeMap::<u128, u128>::new())
            .get(&id)
            .unwrap_or(&0)
    }

    fn balance_of_batch(&self, owners: Vec<Address>, ids: Vec<U128>) -> Vec<U128> {
        assert_eq!(
            owners.len(),
            ids.len(),
            "Owners and Ids must be same length"
        );

        owners
            .iter()
            .zip(ids.iter())
            .map(|(owner, id)| U128::from(self.balance_of(*owner, id.0)))
            .collect()
    }

    fn owned_tokens(&self, owner: Address) -> Vec<(U128, U128)> {
        let issued_tokens = self
            .balance_of
            .get(&owner)
            .unwrap_or_else(|| panic!("Not enough funds"));

        issued_tokens
            .iter()
            .map(|(k, v)| (U128::from(*k), U128::from(*v)))
            .collect()
    }

    fn is_approved_for_all(&self, owner: &Address, operator: &Address) -> bool {
        self.is_approved_for_all
            .get(owner)
            .and_then(|approved_map| approved_map.get(operator))
            .copied()
            .unwrap_or(false)
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

    fn assert_caller_is_approved(&self, from: &Address) {
        let caller_id = l1x_sdk::caller_address();

        if caller_id == *from {
            return;
        }

        let is_approved_operator = self.is_approved_for_all(from, &caller_id);

        assert!(
            is_approved_operator,
            "Not Authorized, the caller is not an approved operator,
            CallerId: {}, From: {}",
            caller_id, from
        );
    }

    fn assert_zero_address(address: &Address) {
        assert_ne!(
            *address,
            Address::from([0; 20]),
            "Zero address is not allowed"
        );
    }

    fn assert_caller_is_owner() {
        assert_eq!(
            caller_address(),
            contract_owner_address(),
            "Only the contract owner can call this method"
        );
    }

    fn assert_not_initialized() {
        match l1x_sdk::storage_read(STORAGE_CONTRACT_KEY) {
            Some(_) => panic!("The contract is already initialized"),
            None => (),
        }
    }
}
