use crate::client;
use crate::errors;

use client::EjudgeClient;
use errors::EjudgeErrors;
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Credentials {
    #[structopt(long = "url")]
    base_url: String,

    #[structopt(short = "id", long)]
    contest_id: String,

    #[structopt(long)]
    username: String,

    #[structopt(long)]
    password: String,
}

pub struct EjudgeLoginClient {
    client: reqwest::Client,
}

impl EjudgeLoginClient {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::builder().cookie_store(true).build()?;
        Ok(EjudgeLoginClient { client: client })
    }

    pub async fn login(
        self,
        credentials: &Credentials,
    ) -> Result<EjudgeClient, Box<dyn std::error::Error>> {
        let login_url = url::Url::parse_with_params(
            &credentials.base_url,
            &[
                ("contest_id", &credentials.contest_id as &str),
                ("login", &credentials.username),
                ("password", &credentials.password),
                ("action_2", "Log in"),
            ],
        )?;

        let logged_in_response = self.client.post(login_url).send().await?;
        println!("Login status : {}", logged_in_response.status());
        println!("Logged in  url : {}", logged_in_response.url());

        let (_, sid_value) = logged_in_response
            .url()
            .query_pairs()
            .find(|(k, _)| k == "SID")
            .ok_or(EjudgeErrors::MissingSessionId)?;

        Ok(EjudgeClient {
            base_url: url::Url::parse(&credentials.base_url)?,
            session_id: sid_value.to_string(),
            client: self.client,
        })
    }
}
