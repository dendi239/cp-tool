mod client;
mod errors;
mod login;
mod submit;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "login", about = "logs you in specified contest.")]
    Login(login::Credentials),

    #[structopt(name = "submit", about = "Submits FILE to given PROBLEM.")]
    Submit(submit::Submission),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match Command::from_args() {
        Command::Login(credentials) => {
            login::EjudgeLoginClient::new()?
                .login(&credentials)
                .await?
                .save_config(std::env::current_dir()?)?;
        }
        Command::Submit(submission) => {
            client::EjudgeClient::from_env()?
                .submit(&submission)
                .await?
        }
    };

    Ok(())
}
