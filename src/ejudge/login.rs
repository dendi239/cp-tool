use super::client::Client;

use crate::errors::{Error, Result};
use crate::login::UserpassCredentials;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct UrlContestIDInfo {
    #[structopt(long = "url")]
    base_url: String,

    #[structopt(short = "id", long)]
    contest_id: String,
}

struct LoginClient {
    client: reqwest::Client,
}

impl LoginClient {
    fn new() -> Result<Self> {
        let client = reqwest::Client::builder().cookie_store(true).build()?;
        Ok(LoginClient { client: client })
    }

    async fn login(
        self,
        contest_info: &UrlContestIDInfo,
        credentials: &UserpassCredentials,
    ) -> Result<Client> {
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
            .ok_or(Error::MissingSessionId)?;

        Ok(Client {
            base_url: url::Url::parse(&contest_info.base_url)?,
            session_id: sid_value.to_string(),
            client: self.client,
        })
    }
}

pub async fn read_login(contest_info: &UrlContestIDInfo) -> Result<Client> {
    LoginClient::new()?
        .login(contest_info, &UserpassCredentials::read()?)
        .await
}
