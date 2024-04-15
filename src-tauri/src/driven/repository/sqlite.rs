#![allow(unused)]
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, pool::PoolConnection, FromRow, Pool, Sqlite};

use crate::{
    config::SqtliteConfig,
    domain::{coin::coin::Coin, settings::settings::Setting, Value},
    driven::repository::{RepoCreateError, Repository},
};

use super::{
    RepoDeleteError, RepoFindAllError, RepoFindOneError, RepoGetAllError, RepoUpdateError,
};

pub(crate) const SQLITE_LOCAL_PATH: &str = "databases";
pub(crate) const SQLITE_FILE: &str = "data.db";

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub(crate) struct CoinSql {
    id: u32,
    name: String,
    symbol: String,
    price: f64,
    volume_24h: f64,
    percent_change_1h: f64,
    percent_change_24h: f64,
    percent_change_7d: f64,
    market_cap: f64,
    last_updated: String,
}

impl From<Coin> for CoinSql {
    fn from(coin: Coin) -> Self {
        Self {
            id: coin.id().value().clone(),
            name: coin.name().value().to_string(),
            symbol: coin.symbol().value().to_string(),
            price: coin.price().value().unwrap_or_default(),
            volume_24h: coin.volume_24h().value().unwrap_or_default(),
            percent_change_1h: coin.percent_change_1h().value().unwrap_or_default(),
            percent_change_24h: coin.percent_change_24h().value().unwrap_or_default(),
            percent_change_7d: coin.percent_change_7d().value().unwrap_or_default(),
            market_cap: coin.market_cap().value().unwrap_or_default(),
            last_updated: coin.last_updated().value().to_string(),
        }
    }
}

impl TryInto<Coin> for CoinSql {
    type Error = &'static str;

    fn try_into(self) -> Result<Coin, Self::Error> {
        Coin::new(
            self.id,
            self.name,
            self.symbol,
            self.price,
            self.volume_24h,
            self.percent_change_1h,
            self.percent_change_24h,
            self.percent_change_7d,
            self.market_cap,
            self.last_updated,
        )
    }
}

enum PoolWrapper {
    Exists(Pool<Sqlite>),
    NotExists,
}

pub(crate) struct SqliteRepository {
    pool: PoolWrapper,
    db_url: String,
    db_path: String,
}

impl SqliteRepository {
    pub fn new(config: &SqtliteConfig) -> Self {
        #[cfg(mobile)]
        let db_path = format!("sqlite:/{}", config.db_path);
        #[cfg(not(mobile))]
        let db_path = format!("{}/{}", SQLITE_LOCAL_PATH, SQLITE_FILE);

        Self {
            pool: PoolWrapper::NotExists,
            db_url: format!("sqlite:{}", db_path),
            db_path
        }
    }

    async fn create_pool(&self) -> Result<Pool<Sqlite>, sqlx::Error> {
        if !sqlx::Sqlite::database_exists(&self.db_url).await? {
            std::fs::create_dir_all(&self.db_path.rsplit_once("/").unwrap().0).unwrap();
            Sqlite::create_database(&self.db_url).await?;
        }

        let pool = Pool::<Sqlite>::connect_lazy(&self.db_url)?;
        sqlx::migrate!("./migrations").run(&pool).await?;
        Ok(pool)
    }

    pub async fn conn(&mut self) -> Result<PoolConnection<Sqlite>, sqlx::Error> {
        use PoolWrapper::*;

        match &self.pool {
            Exists(pool) => pool.acquire().await,
            NotExists => {
                let pool = self.create_pool().await?;
                let conn = pool.acquire().await;

                self.pool = Exists(pool);
                conn
            }
        }
    }
}

impl Repository<Coin, u32> for SqliteRepository {
    async fn create(&mut self, entity: Coin) -> Result<Coin, RepoCreateError> {
        let mut conn = self
            .conn()
            .await
            .map_err(|e| RepoCreateError::Unknown(e.to_string()))?;

        let result = sqlx::query(
            r#"
            INSERT INTO coins (id, name, symbol, price, volume_24h, percent_change_1h, percent_change_24h, percent_change_7d, market_cap, last_updated) 
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(id) DO UPDATE SET
                name = excluded.name,
                symbol = excluded.symbol,
                price = excluded.price,
                volume_24h = excluded.volume_24h,
                percent_change_1h = excluded.percent_change_1h,
                percent_change_24h = excluded.percent_change_24h,
                percent_change_7d = excluded.percent_change_7d,
                market_cap = excluded.market_cap,
                last_updated = excluded.last_updated;
            "#
        )
        .bind(entity.id().value())
        .bind(entity.name().value())
        .bind(entity.symbol().value())
        .bind(entity.price().value())
        .bind(entity.volume_24h().value())
        .bind(entity.percent_change_1h().value())
        .bind(entity.percent_change_24h().value())
        .bind(entity.percent_change_7d().value())
        .bind(entity.market_cap().value())
        .bind(entity.last_updated().value())
        .execute(&mut *conn).await;

        match result {
            Ok(_) => Ok(entity),
            Err(e) => Err(RepoCreateError::Unknown(e.to_string())),
        }
    }

    async fn find_one(&mut self, entity: u32) -> Result<Coin, RepoFindOneError> {
        let mut conn = self
            .conn()
            .await
            .map_err(|e| RepoFindOneError::Unknown(e.to_string()))?;

        let result = sqlx::query_as::<Sqlite, CoinSql>(
            r#"
                    SELECT * FROM coins WHERE id = ?
                    "#,
        )
        .bind(entity)
        .fetch_one(&mut *conn)
        .await;

        match result {
            Ok(coin) => Ok(coin.try_into().unwrap()),
            Err(e) => Err(super::RepoFindOneError::Unknown(e.to_string())),
        }
    }

    async fn find_all(&mut self, entity: u32) -> Result<Vec<Coin>, RepoFindAllError> {
        let mut conn = self
            .conn()
            .await
            .map_err(|e| super::RepoFindAllError::Unknown(e.to_string()))?;

        let result = sqlx::query_as::<Sqlite, CoinSql>(
            r#"
                    SELECT * FROM coins WHERE id = ?
                    "#,
        )
        .bind(entity)
        .fetch_all(&mut *conn)
        .await;

        match result {
            Ok(coins) => Ok(coins
                .into_iter()
                .map(|coin| coin.try_into().unwrap())
                .collect()),
            Err(e) => Err(super::RepoFindAllError::Unknown(e.to_string())),
        }
    }

    async fn update(&mut self, entity: Coin) -> Result<Coin, RepoUpdateError> {
        let mut conn = self
            .conn()
            .await
            .map_err(|e| RepoUpdateError::Unknown(e.to_string()))?;

        let result = sqlx::query(
            r#"
            UPDATE coins
            SET price = ?, volume_24h = ?, percent_change_1h = ?, percent_change_24h = ?, percent_change_7d = ?, market_cap = ?, last_updated = ?
            WHERE id = ?
            "#,
        )
        .bind(entity.price().value())
        .bind(entity.volume_24h().value())
        .bind(entity.percent_change_1h().value())
        .bind(entity.percent_change_24h().value())
        .bind(entity.percent_change_7d().value())
        .bind(entity.market_cap().value())
        .bind(entity.last_updated().value())
        .bind(entity.id().value())
        .execute(&mut *conn)
        .await;

        match result {
            Ok(_) => Ok(entity),
            Err(e) => Err(RepoUpdateError::Unknown(e.to_string())),
        }
    }

    async fn delete(&mut self, entity: u32) -> Result<(), RepoDeleteError> {
        let mut conn = self
            .conn()
            .await
            .map_err(|e| RepoDeleteError::Unknown(e.to_string()))?;

        let result = sqlx::query(
            r#"
                    DELETE FROM coins WHERE id = ?
                    "#,
        )
        .bind(entity)
        .execute(&mut *conn)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RepoDeleteError::Unknown(e.to_string())),
        }
    }

    async fn delete_all(&mut self) -> Result<(), RepoDeleteError> {
        let mut conn = self
            .conn()
            .await
            .map_err(|e| RepoDeleteError::Unknown(e.to_string()))?;

        let result = sqlx::query(
            r#"
                    DELETE FROM coins
                    "#,
        )
        .execute(&mut *conn)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RepoDeleteError::Unknown(e.to_string())),
        }
    }

    async fn get_all(&mut self) -> Result<Vec<Coin>, RepoGetAllError> {
        let mut conn = self
            .conn()
            .await
            .map_err(|e| super::RepoGetAllError::Unknown(e.to_string()))?;

        let result = sqlx::query_as::<Sqlite, CoinSql>(
            r#"
                    SELECT * FROM coins
                    "#,
        )
        .fetch_all(&mut *conn)
        .await;

        match result {
            Ok(coins) => Ok(coins
                .into_iter()
                .map(|coin| coin.try_into().unwrap())
                .collect()),
            Err(e) => Err(super::RepoGetAllError::Unknown(e.to_string())),
        }
    }
}

impl Repository<Setting, String> for SqliteRepository {
    async fn create(&mut self, entity: Setting) -> Result<Setting, RepoCreateError> {
        let mut conn = self
            .conn()
            .await
            .map_err(|e| RepoCreateError::Unknown(e.to_string()))?;

        let result = sqlx::query(
            r#"
            INSERT INTO settings (key, value) 
            VALUES (?, ?)
            ON CONFLICT(key) DO UPDATE SET
                value = excluded.value;
            "#,
        )
        .bind(entity.key().value())
        .bind(entity.value().value())
        .execute(&mut *conn)
        .await;

        match result {
            Ok(_) => Ok(entity),
            Err(e) => Err(RepoCreateError::Unknown(e.to_string())),
        }
    }

    async fn find_one(&mut self, entity: String) -> Result<Setting, RepoFindOneError> {
        let mut conn = self
            .conn()
            .await
            .map_err(|e| RepoFindOneError::Unknown(e.to_string()))?;

        let result = sqlx::query_as::<Sqlite, (String, String)>(
            r#"
                    SELECT * FROM settings WHERE key = ?
                    "#,
        )
        .bind(entity)
        .fetch_one(&mut *conn)
        .await;

        match result {
            Ok((key, value)) => Ok(Setting::new(key, value).unwrap()),
            Err(e) => Err(super::RepoFindOneError::Unknown(e.to_string())),
        }
    }

    async fn find_all(&mut self, entity: String) -> Result<Vec<Setting>, RepoFindAllError> {
        let mut conn = self
            .conn()
            .await
            .map_err(|e| super::RepoFindAllError::Unknown(e.to_string()))?;

        let result = sqlx::query_as::<Sqlite, (String, String)>(
            r#"
                    SELECT * FROM settings WHERE key = ?
                    "#,
        )
        .bind(entity)
        .fetch_all(&mut *conn)
        .await;

        match result {
            Ok(settings) => Ok(settings
                .into_iter()
                .map(|(key, value)| Setting::new(key, value).unwrap())
                .collect()),
            Err(e) => Err(super::RepoFindAllError::Unknown(e.to_string())),
        }
    }

    async fn update(&mut self, entity: Setting) -> Result<Setting, RepoUpdateError> {
        let mut conn = self
            .conn()
            .await
            .map_err(|e| RepoUpdateError::Unknown(e.to_string()))?;

        let result = sqlx::query(
            r#"
            UPDATE settings
            SET value = ?
            WHERE key = ?
            "#,
        )
        .bind(entity.value().value())
        .bind(entity.key().value())
        .execute(&mut *conn)
        .await;

        match result {
            Ok(_) => Ok(entity),
            Err(e) => Err(RepoUpdateError::Unknown(e.to_string())),
        }
    }

    async fn delete(&mut self, entity: String) -> Result<(), RepoDeleteError> {
        let mut conn = self
            .conn()
            .await
            .map_err(|e| RepoDeleteError::Unknown(e.to_string()))?;

        let result = sqlx::query(
            r#"
                    DELETE FROM settings WHERE key = ?
                    "#,
        )
        .bind(entity)
        .execute(&mut *conn)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RepoDeleteError::Unknown(e.to_string())),
        }
    }

    async fn delete_all(&mut self) -> Result<(), RepoDeleteError> {
        let mut conn = self
            .conn()
            .await
            .map_err(|e| RepoDeleteError::Unknown(e.to_string()))?;

        let result = sqlx::query(
            r#"
                    DELETE FROM settings
                    "#,
        )
        .execute(&mut *conn)
        .await;

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(RepoDeleteError::Unknown(e.to_string())),
        }
    }

    async fn get_all(&mut self) -> Result<Vec<Setting>, RepoGetAllError> {
        let mut conn = self
            .conn()
            .await
            .map_err(|e| super::RepoGetAllError::Unknown(e.to_string()))?;

        let result = sqlx::query_as::<Sqlite, (String, String)>(
            r#"
                    SELECT * FROM settings
                    "#,
        )
        .fetch_all(&mut *conn)
        .await;

        match result {
            Ok(settings) => Ok(settings
                .into_iter()
                .map(|(key, value)| Setting::new(key, value).unwrap())
                .collect()),
            Err(e) => Err(super::RepoGetAllError::Unknown(e.to_string())),
        }
    }
}
