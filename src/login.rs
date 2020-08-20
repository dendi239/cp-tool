use crate::ejudge;
use crate::errors;

use errors::Result;
use std::io::Write;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum ContestInfo {
    #[structopt(name = "ejudge")]
    Ejudge(ejudge::UrlContestIDInfo),
}

pub struct UserpassCredentials {
    pub username: String,
    pub password: String,
}

impl UserpassCredentials {
    pub fn read() -> Result<UserpassCredentials> {
        print!("username: ");
        std::io::stdout().flush()?;
        let mut user = String::new();
        std::io::stdin().read_line(&mut user)?;

        let pass = rpassword::prompt_password_stdout("password: ")?;
        Ok(UserpassCredentials {
            username: String::from(user.trim()),
            password: pass,
        })
    }
}
