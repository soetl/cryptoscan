use std::sync::Arc;

use thiserror::Error;
use tokio::sync::Mutex;

use crate::driven::repository::{RepoFindOneError, Repository};

use super::settings::Setting;

#[derive(Debug, Error)]
pub enum FindError {
    #[error("Unknown error: {0}")]
    Unknown(String),
    #[error("Not found")]
    NotFound,
}

pub async fn find_setting<R: Repository<Setting, String>>(
    repository: Arc<Mutex<R>>,
    key: String,
) -> Result<Setting, FindError> {
    repository
        .lock()
        .await
        .find_one(key)
        .await
        .map_err(|e| match e {
            RepoFindOneError::Unknown(e) => FindError::Unknown(e),
            RepoFindOneError::NotFound => FindError::NotFound,
        })
}
