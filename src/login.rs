use crate::client;
use crate::errors;

use client::EjudgeClient;
use errors::{EjudgeErrors, Result};
use std::io::Write;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ContestInfo {
    #[structopt(long = "url")]
    base_url: String,

    #[structopt(short = "id", long)]
    contest_id: String,
}

struct EjudgeCredentials {
    username: String,
    password: String,
}

pub async fn read_login(contest_info: &ContestInfo) -> Result<EjudgeClient> {
    EjudgeLoginClient::new()?
        .login(contest_info, &EjudgeCredentials::read()?)
        .await
}

impl EjudgeCredentials {
    fn read() -> Result<EjudgeCredentials> {
        print!("username: ");
        std::io::stdout().flush()?;
        let mut user = String::new();
        std::io::stdin().read_line(&mut user)?;

        let pass = rpassword::prompt_password_stdout("password: ")?;
        Ok(EjudgeCredentials {
            username: String::from(user.trim()),
            password: pass,
        })
    }
}

pub struct EjudgeLoginClient {
    client: reqwest::Client,
}

impl EjudgeLoginClient {
    pub fn new() -> Result<Self> {
        let client = reqwest::Client::builder().cookie_store(true).build()?;
        Ok(EjudgeLoginClient { client: client })
    }

    async fn login(
        self,
        contest_info: &ContestInfo,
        credentials: &EjudgeCredentials,
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
            .ok_or(EjudgeErrors::MissingSessionId)?;

        Ok(EjudgeClient {
            base_url: url::Url::parse(&contest_info.base_url)?,
            session_id: sid_value.to_string(),
            client: self.client,
        })
    }
}
