#![allow(clippy::use_self)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Top,
    Block,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BidRequst {
    pub chain_id: String,
    pub height: i64,
    pub kind: Kind,
    /// base64 serialized transactions
    pub txs: Vec<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BidResponse {
    pub chain_id: String,
    pub height: i64,
    pub kind: Kind,
    pub tx_hashes: Vec<String>,
}
