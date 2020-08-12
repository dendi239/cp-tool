use serde::{Deserialize, Serialize};
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

struct EjudgeLoginClient {
    client: reqwest::Client,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Config {
    Ejudge {
        contest_url: String,
        session_id: String,
    },
}

impl EjudgeLoginClient {
    fn new() -> Result<Self, Box<dyn Error>> {
        let client = reqwest::Client::builder().cookie_store(true).build()?;
        Ok(EjudgeLoginClient { client: client })
    }

    async fn login(
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

struct EjudgeClient {
    session_id: String,
    base_url: url::Url,
    client: reqwest::Client,
}

impl EjudgeClient {
    fn from_config(config: Config) -> Result<EjudgeClient, Box<dyn Error>> {
        match config {
            Config::Ejudge {
                contest_url: contest_url,
                session_id: session_id,
            } => Ok(EjudgeClient {
                session_id: session_id.clone(),
                base_url: url::Url::parse(&contest_url)?,
                client: reqwest::Client::builder().cookie_store(true).build()?,
            }),
            _ => Err(Box::new(EjudgeErrors::MissingConfig)),
        }
    }

    fn from_env() -> Result<EjudgeClient, Box<dyn Error>> {
        let current_direcrtory = std::env::current_dir()?;
        let curr_dir = current_direcrtory.as_path();
        std::iter::successors(Some(curr_dir), |&x| x.parent())
            .filter_map(|path| std::fs::read(path.join(".cp-tool.config")).ok())
            .filter_map(|file| serde_json::from_slice(&file).ok())
            .filter_map(|config| EjudgeClient::from_config(config).ok())
            .next()
            .ok_or(Box::new(EjudgeErrors::MissingConfig))
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

    fn save_config(&self, path: PathBuf) -> std::io::Result<()> {
        let file_path = path.clone().join(".cp-tool.config");

        let config_string = serde_json::to_string_pretty(&Config::Ejudge {
            contest_url: self.base_url.clone().into_string(),
            session_id: self.session_id.clone(),
        })?;

        println!("filepath to save is: {:?}", file_path);
        println!("Stored config is: {}", config_string);

        std::fs::write(file_path, config_string)
    }
}

#[derive(Debug)]
enum EjudgeErrors {
    MissingConfig,
    MismatchConfig,
    MissingSessionId,
}

impl Display for EjudgeErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EjudgeErrors::MismatchConfig => write!(f, "config type is differ"),
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
            EjudgeLoginClient::new()?
                .login(&credentials)
                .await?
                .save_config(std::env::current_dir()?)?;
        }
        Command::Submit(submission) => EjudgeClient::from_env()?.submit(&submission).await?,
    };

    Ok(())
}
