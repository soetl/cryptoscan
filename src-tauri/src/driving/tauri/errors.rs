#![allow(dead_code)]
use serde::Serialize;
use thiserror::Error;

use crate::domain::settings::settings::SettingError;

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
    #[error("Setting error: {0}")]
    SettingError(#[from] SettingError),
}
