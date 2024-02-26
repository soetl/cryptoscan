use std::sync::Arc;

use thiserror::Error;
use tokio::sync::Mutex;

use crate::driven::repository::{RepoDeleteError, Repository};

use super::coin::Coin;

#[derive(Debug, Error)]
pub enum DeleteError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
    #[error("Coin not found")]
    NotFound,
}

pub async fn delete_coin<R: Repository<Coin, u32>>(
    repository: Arc<Mutex<R>>,
    id: u32,
) -> Result<(), DeleteError> {
    repository
        .lock()
        .await
        .delete(id)
        .await
        .map_err(|e| match e {
            RepoDeleteError::InvalidData(e) => DeleteError::InvalidData(e),
            RepoDeleteError::Unknown(e) => DeleteError::Unknown(e),
            RepoDeleteError::NotFound => DeleteError::NotFound,
        })
}

pub async fn delete_all_coins<R: Repository<Coin, u32>>(
    repository: Arc<Mutex<R>>,
) -> Result<(), DeleteError> {
    repository
        .lock()
        .await
        .delete_all()
        .await
        .map_err(|e| match e {
            RepoDeleteError::InvalidData(e) => DeleteError::InvalidData(e),
            RepoDeleteError::Unknown(e) => DeleteError::Unknown(e),
            RepoDeleteError::NotFound => DeleteError::NotFound,
        })
}
