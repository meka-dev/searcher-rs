use serde::{Deserialize, Serialize};

/// AuctionRequst is used to query the mekatek API if an auction is available for
/// the supplied `chain_id` `height` pair.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuctionRequest {
    pub chain_id: String,
    pub height: u64,
}

/// AuctionResponse contains the payment details of an auction for the supplied
/// `chain_id`, `height` pair. This response will only be returned if an auction
/// was found, otherwise an error will be returned.
///
/// Each bid submitted MUST include payments to the addresses in the `payments`
/// list with the respective allocations and denominations.
/// Please consult the [API docs](https://meka.tech/zenith#payments) for more details.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuctionResponse {
    pub chain_id: String,
    pub height: u64,
    pub payments: Vec<Payment>,
}

/// Payment detail for an auction. The `address` is bech32 encoded and The
/// `allocation` is a float 0 < a <= 1. The `denom` is a standard coin denomination.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Payment {
    pub address: String,
    pub allocation: f64,
    pub denom: String,
}
