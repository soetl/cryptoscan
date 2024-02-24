use std::sync::Arc;

use thiserror::Error;
use tokio::sync::Mutex;

use crate::driven::repository::{RepoGetAllError, Repository};

use super::coin::Coin;

#[derive(Debug, Error)]
pub enum GetAllCoinsError {
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub async fn get_all_coins<R: Repository<Coin, u32>>(
    repository: Arc<Mutex<R>>,
) -> Result<Vec<Coin>, GetAllCoinsError> {
    repository.lock().await.get_all().await.map_err(|e| {
        match e {
            RepoGetAllError::Unknown(e) => GetAllCoinsError::Unknown(e),
        }
    })
}