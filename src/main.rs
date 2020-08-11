use std::error::Error;
use std::fmt::Display;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Credentials {
    #[structopt(long = "url")]
    base_url: String,

    #[structopt(short = "id", long)]
    contest_id: String,

    #[structopt(long)]
    username: String,

    #[structopt(long)]
    password: String,
}

#[derive(Debug, StructOpt)]
struct Submission {
    #[structopt(short, long)]
    problem_id: String,

    #[structopt(short, long, default_value = "3")]
    lang_id: String,

    #[structopt(long, parse(from_os_str))]
    file: PathBuf,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "login", about = "logs you in specified contest.")]
    Login(Credentials),

    #[structopt(name = "submit", about = "Submits FILE to given PROBLEM.")]
    Submit(Submission),
}

struct EjudgeClient {
    client: reqwest::Client,
}

impl EjudgeClient {
    fn new() -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::builder().cookie_store(true).build()?;
        Ok(EjudgeClient { client: client })
    }

    async fn login(
        self,
        credentials: &Credentials,
    ) -> Result<FullClient, Box<dyn std::error::Error>> {
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

        let (_, sid_value) = logged_in_response
            .url()
            .query_pairs()
            .find(|(k, _)| k == "SID")
            .ok_or(EjudgeErrors::MissingSessionId)?;

        Ok(FullClient {
            base_url: url::Url::parse(&credentials.base_url)?,
            session_id: sid_value.to_string(),
            client: self.client,
        })
    }
}

struct FullClient {
    session_id: String,
    base_url: url::Url,
    client: reqwest::Client,
}

impl FullClient {
    // TODO: Scan directories for config
    fn from_env() -> Option<FullClient> {
        None
    }

    async fn submit(&self, submission: &Submission) -> Result<(), Box<dyn Error>> {
        let submit_data: [(&str, &str); 5] = [
            ("prob_id", &submission.problem_id),
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

        println!("Submit response status: {}", submit_response.status());
        Ok(())
    }

    fn save_config(&self, path: PathBuf) {
        // TODO: Put config into file
    }
}

#[derive(Debug)]
enum EjudgeErrors {
    MissingConfig,
    MissingSessionId,
}

impl Display for EjudgeErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EjudgeErrors::MissingConfig => write!(f, "suitable config for ejudge not found"),
            EjudgeErrors::MissingSessionId => {
                write!(f, "missing session_id token in redirected url")
            }
        }
    }
}

impl Error for EjudgeErrors {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Command::from_args() {
        Command::Login(credentials) => {
            EjudgeClient::new()?.login(&credentials).await?;
            // TODO: Save config there.
        }
        Command::Submit(submission) => {
            FullClient::from_env()
                .ok_or(EjudgeErrors::MissingConfig)?
                .submit(&submission)
                .await?;
        }
    };

    Ok(())
}
