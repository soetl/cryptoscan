pub(crate) mod sqlite;

use thiserror::Error;

use crate::domain::Entity;

#[derive(Error, Debug)]
pub(crate) enum RepoCreateError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[derive(Error, Debug)]
pub(crate) enum RepoSelectError {
    #[error("Not found")]
    NotFound,
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[derive(Error, Debug)]
pub(crate) enum RepoFindAllError {
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[derive(Error, Debug)]
pub(crate) enum RepoUpdateError {
    #[error("Not found")]
    NotFound,
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[derive(Error, Debug)]
pub(crate) enum RepoDeleteError {
    #[error("Not found")]
    NotFound,
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub(crate) trait Repository<T, U>
where
    T: Entity,
    U: Entity,
{
    async fn create(&mut self, entity: T) -> Result<T, RepoCreateError>;
    async fn find_one(&mut self, entity: U) -> Result<T, RepoSelectError>;
    async fn find_all(&mut self, entity: U) -> Result<Vec<T>, RepoFindAllError>;
    async fn update(&mut self, entity: T) -> Result<T, RepoUpdateError>;
    async fn delete(&mut self, entity: U) -> Result<(), RepoDeleteError>;
}
