pub mod data_storage_interface;
pub mod name_service_interface;
pub mod nft_interface;

use base64_serde::base64_serde_type;
use borsh::{BorshDeserialize, BorshSerialize};
use l1x_sdk::types::Address;
use serde::{Deserialize, Serialize};

base64_serde_type!(Base64Serde, base64::engine::general_purpose::STANDARD);

#[derive(
    BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
pub struct Cid(pub Address);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
pub struct PublicKey(#[serde(with = "Base64Serde")] pub Vec<u8>);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct Metadata {
    pub name: String,
    pub description: Option<String>,
    pub pub_key: PublicKey,
    pub owner: Address,
    pub versioned: bool,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Serialize, Deserialize)]
pub struct Content(#[serde(with = "Base64Serde")] pub Vec<u8>);

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
pub struct ScoreBoardMetadata {
    pub name: String,
    pub description: Option<String>,
    pub game_id: String,
    pub session_id: String,
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Serialize, Deserialize, Debug)]
pub struct EventData(#[serde(with = "Base64Serde")] pub Vec<u8>);