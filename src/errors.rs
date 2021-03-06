use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("missing config")]
    MissingConfig,

    #[error("missing session id")]
    MissingSessionId,

    #[error("missing problem id")]
    MissingProblemId,

    #[error(transparent)]
    UrlParseError(#[from] url::ParseError),

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ConfigSerializingError(#[from] serde_json::error::Error),

    #[error(transparent)]
    NetworkError(#[from] reqwest::Error),
}
