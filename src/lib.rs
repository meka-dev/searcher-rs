#![warn(clippy::nursery, clippy::cargo)]

use async_trait::async_trait;
use reqwest::{Client, ClientBuilder, Url};

mod auction;
pub use auction::{AuctionRequst, AuctionResponse};

mod bid;
pub use bid::{BidRequst, BidResponse, Kind as BidKind};

mod error;
pub use error::AuctionError;

static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

#[async_trait::async_trait]
pub trait Builder: Send + Sync {
    async fn bid(
        &self,
        chain_id: String,
        height: i64,
        kind: bid::Kind,
        txs: Vec<Vec<u8>>,
    ) -> Result<bid::BidResponse, error::Error>;

    async fn auction(
        &self,
        chain_id: String,
        height: i64,
    ) -> Result<auction::AuctionResponse, error::Error>;
}

#[derive(Clone)]
pub struct Http {
    base_url: Url,
    client: Client,
}

impl Http {
    pub fn new(url: String) -> Result<Self, error::Error> {
        let base_url = Url::parse(&url).map_err(|e| error::Error::Init(e.to_string()))?;
        let client = ClientBuilder::new().user_agent(USER_AGENT).build()?;

        Ok(Self { base_url, client })
    }

    pub fn new_client(url: String, client: Client) -> Result<Self, error::Error> {
        let base_url = Url::parse(&url).map_err(|e| error::Error::Init(e.to_string()))?;

        Ok(Self { base_url, client })
    }
}

#[async_trait]
impl Builder for Http {
    async fn bid(
        &self,
        chain_id: String,
        height: i64,
        kind: bid::Kind,
        txs: Vec<Vec<u8>>,
    ) -> Result<bid::BidResponse, error::Error> {
        let req = bid::BidRequst {
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

    async fn auction(
        &self,
        chain_id: String,
        height: i64,
    ) -> Result<auction::AuctionResponse, error::Error> {
        let req = auction::AuctionRequst { chain_id, height };
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
