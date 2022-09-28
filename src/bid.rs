#![allow(clippy::use_self)]

use serde::{Deserialize, Serialize};

/// Bidding is allowed to target either the top of the block or any position in
/// the block.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    Top,
    Block,
}

/// The BidRequest signals intent to bid on the `kind` at `height` for the given
/// `chain_id`. The bid is implicit in the payment transactions/messages included
/// in the `txs` list.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BidRequest {
    pub chain_id: String,
    pub height: u64,
    pub kind: Kind,
    /// base64 serialized transactions
    pub txs: Vec<Vec<u8>>,
}

/// A BidResponse is the indication that the request well formed and considered
/// for the given `chain_id` and `height`, but it does not imply guaranteed inclusion.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct BidResponse {
    pub chain_id: String,
    pub height: u64,
    pub kind: Kind,
    pub tx_hashes: Vec<String>,
}
