//! Strike client

use std::time::Duration;

use reqwest::header::{ACCEPT, AUTHORIZATION, HeaderMap, HeaderValue};
use reqwest::{Client, Method, Response};
use serde::de::DeserializeOwned;
use url::Url;

use crate::auth::StrikeAuth;
use crate::constant::{API_ROOT_URL, BTC_TICKER, USER_AGENT_NAME};
use crate::error::Error;
use crate::response::Balance;

/// Strike client
#[derive(Debug, Clone)]
pub struct StrikeClient {
    /// Root URL for the API.
    root_url: Url,
    /// HTTP client.
    client: Client,
    /// Authentication
    auth: StrikeAuth,
}

impl StrikeClient {
    /// Construct a new client.
    pub fn new(auth: StrikeAuth) -> Result<Self, Error> {
        Ok(Self {
            root_url: Url::parse(API_ROOT_URL)?,
            client: Client::builder()
                .user_agent(USER_AGENT_NAME)
                .timeout(Duration::from_secs(25))
                .build()?,
            auth,
        })
    }

    async fn call_api<T>(&self, method: Method, url: Url) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        // Build headers
        let mut headers: HeaderMap = HeaderMap::with_capacity(2);
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));

        match &self.auth {
            StrikeAuth::ApiKey(key) => {
                let val: String = format!("Bearer {key}");

                let mut value: HeaderValue = HeaderValue::from_str(&val)?;
                value.set_sensitive(true);

                headers.insert(AUTHORIZATION, value);
            }
        }

        // Send request
        let response: Response = self
            .client
            .request(method, url)
            .headers(headers)
            .send()
            .await?;

        // Propagate error if any
        let response: Response = response.error_for_status()?;

        // Deserialize response
        Ok(response.json().await?)
    }

    /// Get **bitcoin** balance.
    pub async fn balance(&self) -> Result<Balance, Error> {
        let url: Url = self.root_url.join("balances")?;

        // Get balances
        let balances: Vec<Balance> = self.call_api(Method::GET, url).await?;

        // Find balance for BTC
        let balance: Balance = balances
            .into_iter()
            .find(|b| b.currency == BTC_TICKER)
            .unwrap_or_else(|| Balance::new(BTC_TICKER));

        Ok(balance)
    }
}
