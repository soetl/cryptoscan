use std::sync::Arc;

use thiserror::Error;
use tokio::sync::Mutex;

use crate::{driven::repository::{RepoUpdateError, Repository}, driving::tauri::coins::CoinResponse};

use super::coin::Coin;

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
    #[error("Coin not found")]
    NotFound,
}

pub async fn update_coin<R: Repository<Coin, u32>>(repository: Arc<Mutex<R>>, coin: CoinResponse) -> Result<Coin, UpdateError> {
    let coin = Coin::from(coin);

    repository.lock().await.update(coin).await.map_err(|e| {
        match e {
            RepoUpdateError::InvalidData(e) => UpdateError::InvalidData(e),
            RepoUpdateError::Unknown(e) => UpdateError::Unknown(e),
            RepoUpdateError::NotFound => UpdateError::NotFound,
        }
    })
}