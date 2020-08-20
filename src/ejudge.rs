use crate::client;
use crate::errors;
use crate::login;

use async_trait::async_trait;
use client::{Client, Submission};
use errors::Result;
use login::UserpassCredentials;
use serde::{Deserialize, Serialize};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct UrlContestIDInfo {
    #[structopt(long = "url")]
    pub base_url: String,

    #[structopt(short = "id", long)]
    pub contest_id: String,
}

pub struct EjudgeClient {
    pub session_id: String,
    pub base_url: url::Url,
    pub client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    contest_url: String,
    session_id: String,
}

struct EjudgeLoginClient {
    client: reqwest::Client,
}

impl EjudgeLoginClient {
    fn new() -> Result<Self> {
        let client = reqwest::Client::builder().cookie_store(true).build()?;
        Ok(EjudgeLoginClient { client: client })
    }

    async fn login(
        self,
        contest_info: &UrlContestIDInfo,
        credentials: &UserpassCredentials,
    ) -> Result<EjudgeClient> {
        let login_url = url::Url::parse_with_params(
            &contest_info.base_url,
            &[
                ("contest_id", &contest_info.contest_id as &str),
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
            .ok_or(errors::Error::MissingSessionId)?;

        Ok(EjudgeClient {
            base_url: url::Url::parse(&contest_info.base_url)?,
            session_id: sid_value.to_string(),
            client: self.client,
        })
    }
}

pub async fn read_login(contest_info: &UrlContestIDInfo) -> Result<EjudgeClient> {
    EjudgeLoginClient::new()?
        .login(contest_info, &login::UserpassCredentials::read()?)
        .await
}

#[async_trait]
impl Client for EjudgeClient {
    type Config = Config;

    fn from_config(config: Config) -> Result<EjudgeClient> {
        Ok(EjudgeClient {
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

    async fn submit(&self, submission: &Submission) -> Result<()> {
        let pid = &client::get_problem_id(submission)?;

        let submit_data: [(&str, &str); 5] = [
            ("prob_id", pid),
            ("lang_id", &submission.lang_id),
            ("file", &std::fs::read_to_string(&submission.file)?),
            ("SID", &self.session_id),
            ("action_40", "Send!"),
        ];

        let submit_response = self
            .client
            .post(self.base_url.clone())
            .form(&submit_data)
            .send()
            .await?;

        // TODO: Check if session_id is valid.
        //       There're must be sid as query parameter in response.
        println!("Submit response url: {}", submit_response.url());
        open::that(submit_response.url().clone().into_string())?;

        Ok(())
    }
}
