use std::fmt::{Display, Formatter};

use serde::{de::StdError, Deserialize, Serialize};
use thiserror::Error;

/// Collection of all errors returned by the library.
#[derive(Debug, Error)]
pub enum Error {
    #[error("error: {:?}, code: {:?}, text: {:?}", .0.error, .0.status_code, .0.status_text)]
    Auction(AuctionError),
    #[error("failed to call api")]
    Transport(#[from] reqwest::Error),
    #[error(transparent)]
    Parse(#[from] url::ParseError),
    #[error("failed to init builder client")]
    Init(String),
}

/// AuctionError is the wrapper for all errors returned by the mekatek API.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct AuctionError {
    pub error: String,
    pub status_code: u64,
    pub status_text: String,
}

impl Display for AuctionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "error: {}, code: {}, text: {}",
            self.error, self.status_code, self.status_text
        )
    }
}

impl StdError for AuctionError {}
