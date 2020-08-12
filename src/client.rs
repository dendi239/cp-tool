use crate::errors;

use errors::EjudgeErrors;
use errors::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Config {
    Ejudge {
        contest_url: String,
        session_id: String,
    },
}

pub struct EjudgeClient {
    pub session_id: String,
    pub base_url: url::Url,
    pub client: reqwest::Client,
}

impl EjudgeClient {
    pub fn from_config(config: Config) -> Result<EjudgeClient> {
        match config {
            Config::Ejudge {
                contest_url,
                session_id,
            } => Ok(EjudgeClient {
                session_id: session_id.clone(),
                base_url: url::Url::parse(&contest_url)?,
                client: reqwest::Client::builder().cookie_store(true).build()?,
            }),
        }
    }

    pub fn from_env() -> Result<EjudgeClient> {
        let current_direcrtory = std::env::current_dir()?;
        let curr_dir = current_direcrtory.as_path();
        std::iter::successors(Some(curr_dir), |&x| x.parent())
            .filter_map(|path| std::fs::read(path.join(".cp-tool.config")).ok())
            .filter_map(|file| serde_json::from_slice(&file).ok())
            .filter_map(|config| EjudgeClient::from_config(config).ok())
            .next()
            .ok_or(Box::new(EjudgeErrors::MissingConfig))
    }

    pub fn save_config(&self, path: std::path::PathBuf) -> std::io::Result<()> {
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
