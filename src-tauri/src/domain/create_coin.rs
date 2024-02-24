use std::sync::Arc;

use thiserror::Error;
use tokio::sync::Mutex;

use crate::{
    driven::repository::{RepoCreateError, Repository},
    driving::tauri::coins::CreateCoinRequest,
};

use super::coin::Coin;

#[derive(Debug, Error)]
pub enum CreateError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub async fn create_coin<R: Repository<Coin, u32>>(
    repository: Arc<Mutex<R>>,
    coin: CreateCoinRequest,
) -> Result<Coin, CreateError> {
    let coin = Coin::from(coin);

    repository.lock().await.create(coin).await.map_err(|e| {
        match e {
            RepoCreateError::InvalidData(e) => CreateError::InvalidData(e),
            RepoCreateError::Unknown(e) => CreateError::Unknown(e),
        }
    })
}
