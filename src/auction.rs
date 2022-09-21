use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuctionRequst {
    pub chain_id: String,
    pub height: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuctionResponse {
    pub chain_id: String,
    pub height: i64,
    pub payments: Vec<Payment>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Payment {
    pub address: String,
    pub allocation: f64,
    pub denom: String,
}
