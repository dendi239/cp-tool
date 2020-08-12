pub use std::error::Error;
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub enum EjudgeErrors {
    MissingConfig,
    MissingSessionId,
}

impl std::fmt::Display for EjudgeErrors {
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
