use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::State;
use validator::Validate;

use crate::{
    domain::{self, coin::coin::Coin, Value},
    fetch::coinmarketcap::{fetch_ids, fetch_symbols},
    AppState,
};

use super::errors::TauriErrors;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateCoinRequest {
    #[validate(range(min = 1, message = "id must be greater than 0"))]
    pub id: u32,
    #[validate(length(min = 1, message = "name must be at least 1 character"))]
    pub name: String,
    #[validate(length(
        min = 1,
        max = 255,
        message = "symbol must be between 1 and 255 characters"
    ))]
    pub symbol: String,
    #[validate(range(min = 0, message = "price must be greater than or equal to 0"))]
    pub price: Option<f64>,
    #[validate(range(min = 0, message = "volume_24h must be greater than or equal to 0"))]
    pub volume_24h: Option<f64>,
    pub percent_change_1h: Option<f64>,
    pub percent_change_24h: Option<f64>,
    pub percent_change_7d: Option<f64>,
    #[validate(range(min = 0, message = "market_cap must be greater than or equal to 0"))]
    pub market_cap: Option<f64>,
    #[validate(length(min = 20, message = "last_updated must be at least 20 characters"))]
    pub last_updated: String,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateCoinsRequest {
    #[validate(length(min = 1, message = "coins must have at least 1 coin"))]
    pub coins: Vec<CreateCoinRequest>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct UpdateCoinsRequest {
    #[validate(length(min = 1, message = "ids must have at least 1 id"))]
    pub ids: Vec<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct FindCoinRequest {
    #[validate(range(min = 1, message = "id must be greater than 0"))]
    pub id: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct DeleteCoinRequest {
    #[validate(range(min = 1, message = "id must be greater than 0"))]
    pub id: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct FetchCoinsByIdRequest {
    #[validate(length(min = 1, message = "ids must have at least 1 id"))]
    pub ids: Vec<u32>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct FetchCoinsBySymbolRequest {
    #[validate(length(min = 1, max = 255, message = "symbols must have at least 1 symbol"))]
    pub symbols: Vec<String>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct CoinResponse {
    pub id: u32,
    pub name: String,
    pub symbol: String,
    pub price: f64,
    pub volume_24h: f64,
    pub percent_change_1h: f64,
    pub percent_change_24h: f64,
    pub percent_change_7d: f64,
    pub market_cap: f64,
    pub last_updated: String,
}

impl<'de> Deserialize<'de> for CoinResponse {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct CoinMarketCapData {
            id: u32,
            name: String,
            symbol: String,
            quote: Quote,
        }

        #[derive(Clone, Deserialize, Serialize, Debug)]
        struct Quote {
            #[serde(rename = "USD")]
            usd: QuoteData,
        }

        #[derive(Clone, Deserialize, Serialize, Debug)]
        struct QuoteData {
            price: Option<f64>,
            volume_24h: Option<f64>,
            percent_change_1h: Option<f64>,
            percent_change_24h: Option<f64>,
            percent_change_7d: Option<f64>,
            market_cap: Option<f64>,
            last_updated: String,
        }

        let temp = CoinMarketCapData::deserialize(deserializer)?;
        Ok(CoinResponse {
            id: temp.id,
            name: temp.name,
            symbol: temp.symbol,
            price: temp.quote.usd.price.unwrap_or_default(),
            volume_24h: temp.quote.usd.volume_24h.unwrap_or_default(),
            percent_change_1h: temp.quote.usd.percent_change_1h.unwrap_or_default(),
            percent_change_24h: temp.quote.usd.percent_change_24h.unwrap_or_default(),
            percent_change_7d: temp.quote.usd.percent_change_7d.unwrap_or_default(),
            market_cap: temp.quote.usd.market_cap.unwrap_or_default(),
            last_updated: temp.quote.usd.last_updated,
        })
    }
}

impl From<Coin> for CoinResponse {
    fn from(coin: Coin) -> Self {
        CoinResponse {
            id: coin.id().value().to_owned(),
            name: coin.name().value().to_string().clone(),
            symbol: coin.symbol().value().to_string().clone(),
            price: coin.price().value().unwrap_or(0.0),
            volume_24h: coin.volume_24h().value().unwrap_or(0.0),
            percent_change_1h: coin.percent_change_1h().value().unwrap_or(0.0),
            percent_change_24h: coin.percent_change_24h().value().unwrap_or(0.0),
            percent_change_7d: coin.percent_change_7d().value().unwrap_or(0.0),
            market_cap: coin.market_cap().value().unwrap_or(0.0),
            last_updated: coin.last_updated().value().to_string().clone(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq)]
pub struct CoinsResponse {
    pub coins: Vec<CoinResponse>,
}

impl From<Vec<Coin>> for CoinsResponse {
    fn from(coins: Vec<Coin>) -> Self {
        CoinsResponse {
            coins: coins.into_iter().map(|coin| coin.into()).collect(),
        }
    }
}

#[tauri::command]
pub(crate) async fn create_coin(
    request: CreateCoinRequest,
    state: State<'_, Arc<AppState>>,
) -> Result<String, TauriErrors> {
    request.validate()?;

    let result = domain::coin::create_coin::create_coin(state.sqlite_repo.clone(), request).await;

    match result {
        Ok(coin) => Ok(serde_json::to_string(&CoinResponse::from(coin)).unwrap()),
        Err(e) => Err(TauriErrors::UnknownError(e.to_string())),
    }
}

#[tauri::command]
pub(crate) async fn create_coins(
    request: CreateCoinsRequest,
    state: State<'_, Arc<AppState>>,
) -> Result<String, TauriErrors> {
    request.validate()?;

    let mut result = vec![];

    for coin in request.coins {
        match domain::coin::create_coin::create_coin(state.sqlite_repo.clone(), coin).await {
            Ok(coin) => result.push(CoinResponse::from(coin)),
            Err(e) => return Err(TauriErrors::UnknownError(e.to_string())),
        }
    }

    Ok(serde_json::to_string(&result).unwrap())
}

#[tauri::command]
pub(crate) async fn update_coins(
    request: UpdateCoinsRequest,
    state: State<'_, Arc<AppState>>,
) -> Result<String, TauriErrors> {
    request.validate()?;

    let store = state.store.lock().await;

    let Some(token_value) = store.get("token") else {
        return Err(TauriErrors::TokenNotSpecified);
    };

    let Some(token) = token_value.as_str() else {
        return Err(TauriErrors::TokenNotValid);
    };

    let result = fetch_ids(request.ids, token.to_string()).await;

    match result {
        Ok(coins) => {
            let mut result = vec![];

            for coin in coins.coins {
                match domain::coin::update_coin::update_coin(state.sqlite_repo.clone(), coin).await
                {
                    Ok(coin) => result.push(CoinResponse::from(coin)),
                    Err(e) => return Err(TauriErrors::UnknownError(e.to_string())),
                }
            }

            Ok(serde_json::to_string(&result).unwrap())
        }
        Err(e) => Err(TauriErrors::UnknownError(e)),
    }
}

#[tauri::command]
pub(crate) async fn find_coin(
    request: FindCoinRequest,
    state: State<'_, Arc<AppState>>,
) -> Result<String, TauriErrors> {
    request.validate()?;

    match domain::coin::find_coin::find_coin(state.sqlite_repo.clone(), request.id).await {
        Ok(coin) => Ok(serde_json::to_string(&CoinResponse::from(coin)).unwrap()),
        Err(e) => Err(TauriErrors::UnknownError(e.to_string())),
    }
}

#[tauri::command]
pub(crate) async fn find_coins(
    request: FindCoinRequest,
    state: State<'_, Arc<AppState>>,
) -> Result<String, TauriErrors> {
    request.validate()?;

    match domain::coin::find_coin::find_coins(state.sqlite_repo.clone(), request.id).await {
        Ok(coins) => Ok(serde_json::to_string(&CoinsResponse::from(coins).coins).unwrap()),
        Err(e) => Err(TauriErrors::UnknownError(e.to_string())),
    }
}

#[tauri::command]
pub(crate) async fn delete_coin(
    request: DeleteCoinRequest,
    state: State<'_, Arc<AppState>>,
) -> Result<(), TauriErrors> {
    request.validate()?;

    match domain::coin::delete_coin::delete_coin(state.sqlite_repo.clone(), request.id).await {
        Ok(_) => Ok(()),
        Err(e) => Err(TauriErrors::UnknownError(e.to_string())),
    }
}

#[tauri::command]
pub(crate) async fn delete_all_coins(state: State<'_, Arc<AppState>>) -> Result<(), TauriErrors> {
    match domain::coin::delete_coin::delete_all_coins(state.sqlite_repo.clone()).await {
        Ok(_) => Ok(()),
        Err(e) => Err(TauriErrors::UnknownError(e.to_string())),
    }
}

#[tauri::command]
pub(crate) async fn get_all_coins(state: State<'_, Arc<AppState>>) -> Result<String, TauriErrors> {
    match domain::coin::get_all_coins::get_all_coins(state.sqlite_repo.clone()).await {
        Ok(coins) => Ok(serde_json::to_string(&CoinsResponse::from(coins).coins).unwrap()),
        Err(e) => Err(TauriErrors::UnknownError(e.to_string())),
    }
}

#[tauri::command]
pub(crate) async fn fetch_coins_by_id(
    request: FetchCoinsByIdRequest,
    state: State<'_, Arc<AppState>>,
) -> Result<String, TauriErrors> {
    request.validate()?;

    let store = state.store.lock().await;

    let Some(token_value) = store.get("token") else {
        return Err(TauriErrors::TokenNotSpecified);
    };

    let Some(token) = token_value.as_str() else {
        return Err(TauriErrors::TokenNotValid);
    };

    let result = fetch_ids(request.ids, token.to_string()).await;

    match result {
        Ok(coins) => Ok(serde_json::to_string(&coins.coins).unwrap()),
        Err(e) => Err(TauriErrors::UnknownError(e)),
    }
}

#[tauri::command]
pub(crate) async fn fetch_coins_by_symbol(
    request: FetchCoinsBySymbolRequest,
    state: State<'_, Arc<AppState>>,
) -> Result<String, TauriErrors> {
    request.validate()?;

    let store = state.store.lock().await;

    let Some(token_value) = store.get("token") else {
        return Err(TauriErrors::TokenNotSpecified);
    };

    let Some(token) = token_value.as_str() else {
        return Err(TauriErrors::TokenNotValid);
    };

    let result = fetch_symbols(request.symbols, token.to_string()).await;

    match result {
        Ok(coins) => Ok(serde_json::to_string(&coins.coins).unwrap()),
        Err(e) => Err(TauriErrors::UnknownError(e)),
    }
}

#[tauri::command]
pub(crate) async fn set_cmc_token(
    token: String,
    state: State<'_, Arc<AppState>>,
) -> Result<(), TauriErrors> {
    if token.is_empty() {
        return Err(TauriErrors::TokenNotValid);
    }

    match state
        .store
        .lock()
        .await
        .insert("token".to_string(), json!(token))
    {
        Ok(_) => Ok(()),
        Err(_) => {
            return Err(TauriErrors::UnknownError("Error setting token".to_string()));
        }
    }
}
