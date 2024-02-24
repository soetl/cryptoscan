use std::sync::Arc;

use thiserror::Error;
use tokio::sync::Mutex;

use crate::driven::repository::{RepoFindAllError, RepoFindOneError, Repository};

use super::coin::Coin;

#[derive(Debug, Error)]
pub enum FindError {
    #[error("Unknown error: {0}")]
    Unknown(String),
    #[error("Not found")]
    NotFound,
}

pub async fn find_coin<R: Repository<Coin, u32>>(
    repository: Arc<Mutex<R>>,
    id: u32,
) -> Result<Coin, FindError> {
    repository.lock().await.find_one(id).await.map_err(|e| {
        match e {
            RepoFindOneError::Unknown(e) => FindError::Unknown(e),
            RepoFindOneError::NotFound => FindError::NotFound,
        }
    })
}

pub async fn find_coins<R: Repository<Coin, u32>>(
    repository: Arc<Mutex<R>>,
    id: u32,
) -> Result<Vec<Coin>, FindError> {
    repository.lock().await.find_all(id).await.map_err(|e| {
        match e {
            RepoFindAllError::Unknown(e) => FindError::Unknown(e),
        }
    })
}