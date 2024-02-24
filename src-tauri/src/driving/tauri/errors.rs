use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum TauriErrors {
    #[error("Unknown error: {0}")]
    UnknownError(String),
    #[error("Error validating: {0}")]
    ValidateError(#[from] validator::ValidationErrors),
    #[error("CoinMarketCap token not specified")]
    TokenNotSpecified,
    #[error("CoinMarketCap token not valid")]
    TokenNotValid,
}
