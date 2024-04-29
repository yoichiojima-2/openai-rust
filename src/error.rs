use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("API key not set in environment")]
    ApiKeyNotSet,
    #[error("Request failed: {0}")]
    ReqwestError(#[from] reqwest::Error),
}
