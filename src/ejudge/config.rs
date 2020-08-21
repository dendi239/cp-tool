use crate::client::{AsClientConfig, ClientConfig, ConfigClient};
use crate::errors::Result;

use super::client::Client;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    contest_url: String,
    session_id: String,
}

impl AsClientConfig for Config {
    fn as_client_config(self) -> ClientConfig {
        ClientConfig::Ejudge(self)
    }
}

#[async_trait]
impl ConfigClient for Client {
    type Config = Config;

    fn from_config(config: Config) -> Result<Client> {
        Ok(Client {
            session_id: config.session_id.clone(),
            base_url: url::Url::parse(&config.contest_url)?,
            client: reqwest::Client::builder().cookie_store(true).build()?,
        })
    }

    fn get_config(&self) -> Config {
        Config {
            contest_url: self.base_url.clone().into_string(),
            session_id: self.session_id.clone(),
        }
    }
}
