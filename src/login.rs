use crate::errors;

use errors::Result;
use std::io::Write;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct ContestInfo {
    #[structopt(long = "url")]
    pub base_url: String,

    #[structopt(short = "id", long)]
    pub contest_id: String,
}

pub struct EjudgeCredentials {
    pub username: String,
    pub password: String,
}

impl EjudgeCredentials {
    pub fn read() -> Result<EjudgeCredentials> {
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
