use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;

use crate::{driven::repository::{RepoCreateError, Repository}, driving::tauri::settings::CreateSettingRequest};

use super::settings::{Setting, SettingError};

#[derive(Debug, Error)]
pub enum CreateError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
    #[error("Setting error: {0}")]
    SettingError(#[from] SettingError),
}

pub async fn create_setting<R: Repository<Setting, String>>(
    repository: Arc<Mutex<R>>,
    setting: CreateSettingRequest,
) -> Result<Setting, CreateError> {
    let setting = Setting::try_from(setting)?;

    repository
        .lock()
        .await
        .create(setting)
        .await
        .map_err(|e| match e {
            RepoCreateError::InvalidData(e) => CreateError::InvalidData(e),
            RepoCreateError::Unknown(e) => CreateError::Unknown(e),
        })
}
