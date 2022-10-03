//! Bid on blockspace via the mekatek block auctions.
//!
//! This crate provides a simple wrapper around the Zenith block Builder
//! API. It allows anyone to bid on block space auctions and poll for available
//! auctions coming up.
//!
//! # Usage
//!
//! Add `searcher-rs` to the dependencies section of your `Cargo.toml` file.
//!
//! ```toml
//! [dependencies]
//! mekatek-searcher-rs = "1"
//! ```
#![warn(clippy::nursery, clippy::cargo)]

use async_trait::async_trait;
use reqwest::{Client, ClientBuilder, Url};

mod auction;
pub use auction::{AuctionRequest, AuctionResponse};

mod bid;
pub use bid::{BidRequest, BidResponse, Kind as BidKind};

mod error;
pub use error::AuctionError;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

/// The Builder trait encapsulates the capabilities of the Zenith builder API.
#[async_trait::async_trait]
pub trait Builder: Send + Sync {
    async fn bid(
        &self,
        chain_id: String,
        height: u64,
        kind: bid::Kind,
        txs: Vec<Vec<u8>>,
    ) -> Result<bid::BidResponse, error::Error>;

    async fn auction(
        &self,
        chain_id: String,
        height: u64,
    ) -> Result<auction::AuctionResponse, error::Error>;
}

/// The builder API exposed via HTTP.
#[derive(Clone)]
pub struct Http {
    base_url: Url,
    client: Client,
}

impl Http {
    /// Instantiate a new HTTP client for the given URL, e.g. `api.mekatek.xyz`.
    ///
    /// This function can error when parsing the URL or if the underlying client
    /// cannot be instantiated.
    pub fn new(url: String) -> Result<Self, error::Error> {
        let base_url = Url::parse(&url).map_err(|e| error::Error::Init(e.to_string()))?;
        let client = ClientBuilder::new().user_agent(USER_AGENT).build()?;

        Ok(Self { base_url, client })
    }

    /// Instantiate a new HTTP client for the given URL, e.g. `api.mekatek.xyz`
    /// using the user supplied reqwest client.
    ///
    /// This function can error when parsing the URL.
    pub fn new_client(url: String, client: Client) -> Result<Self, error::Error> {
        let base_url = Url::parse(&url).map_err(|e| error::Error::Init(e.to_string()))?;

        Ok(Self { base_url, client })
    }
}

#[async_trait]
impl Builder for Http {
    /// Bid on an auction of `kind` at `height` on `chain_id` with the given list
    /// of `txs`. The `txs` MUST be base64 encoded. The `height` cannot be too
    /// far into the future (consult the [API docs](https://meka.tech/zenith#payments)
    /// for current values).
    ///
    /// This function can error when the underlying transport or the mekatek API
    /// fails. In the latter case the `AuctionError` will contain details.
    async fn bid(
        &self,
        chain_id: String,
        height: u64,
        kind: bid::Kind,
        txs: Vec<Vec<u8>>,
    ) -> Result<bid::BidResponse, error::Error> {
        let req = bid::BidRequest {
            chain_id,
            height,
            kind,
            txs,
        };
        let res = self
            .client
            .post(self.base_url.join("v0/bid")?)
            .json(&req)
            .send()
            .await
            .map_err(error::Error::Transport)?;

        if !res.status().is_success() {
            return Err(error::Error::Auction(res.json::<AuctionError>().await?));
        }

        Ok(res.json::<bid::BidResponse>().await?)
    }

    /// Retrieve details of upcoming auctions at `height` on `chain_id`. The
    /// `height` cannot be too far into the future (consult the [API docs](https://meka.tech/zenith#payments)
    /// for current values).
    ///
    /// This function can error when the underlying transport or the mekatek API
    /// fails. In the latter case the `AuctionError` will contain details.
    async fn auction(
        &self,
        chain_id: String,
        height: u64,
    ) -> Result<auction::AuctionResponse, error::Error> {
        let req = auction::AuctionRequest { chain_id, height };
        let res = self
            .client
            .get(self.base_url.join("v0/auction")?)
            .json(&req)
            .send()
            .await
            .map_err(error::Error::Transport)?;

        if !res.status().is_success() {
            return Err(error::Error::Auction(res.json::<AuctionError>().await?));
        }

        Ok(res.json::<auction::AuctionResponse>().await?)
    }
}

#[cfg(test)]
mod tests {
    use wiremock::{
        matchers::{method, path},
        Mock,
        MockServer,
        ResponseTemplate,
    };

    use super::{error, Builder, Http};

    #[tokio::test]
    async fn auction_gone() -> Result<(), Box<dyn std::error::Error>> {
        let height = 5994269;
        let response = ResponseTemplate::new(410)
            .set_body_bytes(include_bytes!("../fixtures/auction_gone.json").to_vec());

        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .and(path("v0/auction"))
            .respond_with(response)
            .mount(&server)
            .await;

        let http = Http::new(server.uri())?;
        let p = http.auction("osmosis-1".to_string(), height).await;

        assert!(matches!(
            p,
            Err(error::Error::Auction(error::AuctionError {
                status_code: 410,
                ..
            }))
        ));

        Ok(())
    }
}
