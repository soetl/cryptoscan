use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    domain::{Entity, Value},
    driving::tauri::coins::{CoinResponse, CreateCoinRequest},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoinId(u32);

impl Value for CoinId {
    type ValueType = u32;

    fn value(&self) -> &Self::ValueType {
        &self.0
    }
}

impl TryFrom<u32> for CoinId {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value > 0 {
            Ok(Self(value))
        } else {
            Err("Coin ID must be greater than 0")
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoinName(String);

impl Value for CoinName {
    type ValueType = String;

    fn value(&self) -> &Self::ValueType {
        &self.0
    }
}

impl TryFrom<String> for CoinName {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("Coin name must not be empty")
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CoinSymbol(String);

impl Value for CoinSymbol {
    type ValueType = String;

    fn value(&self) -> &Self::ValueType {
        &self.0
    }
}

impl TryFrom<String> for CoinSymbol {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("Coin symbol must not be empty")
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinPrice(Option<f64>);

impl Value for CoinPrice {
    type ValueType = Option<f64>;

    fn value(&self) -> &Self::ValueType {
        &self.0
    }
}

impl TryFrom<f64> for CoinPrice {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value == f64::default()
            || value.is_nan()
            || value.is_infinite()
            || value.is_sign_negative()
        {
            Ok(Self(None))
        } else {
            Ok(Self(Some(value)))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinVolume24h(Option<f64>);

impl Value for CoinVolume24h {
    type ValueType = Option<f64>;

    fn value(&self) -> &Self::ValueType {
        &self.0
    }
}

impl TryFrom<f64> for CoinVolume24h {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value == f64::default()
            || value.is_nan()
            || value.is_infinite()
            || value.is_sign_negative()
        {
            Ok(Self(None))
        } else {
            Ok(Self(Some(value)))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinPercentChange1h(Option<f64>);

impl Value for CoinPercentChange1h {
    type ValueType = Option<f64>;

    fn value(&self) -> &Self::ValueType {
        &self.0
    }
}

impl TryFrom<f64> for CoinPercentChange1h {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value == f64::default() || value.is_nan() || value.is_infinite() {
            Ok(Self(None))
        } else {
            Ok(Self(Some(value)))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinPercentChange24h(Option<f64>);

impl Value for CoinPercentChange24h {
    type ValueType = Option<f64>;

    fn value(&self) -> &Self::ValueType {
        &self.0
    }
}

impl TryFrom<f64> for CoinPercentChange24h {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value == f64::default() || value.is_nan() || value.is_infinite() {
            Ok(Self(None))
        } else {
            Ok(Self(Some(value)))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinPercentChange7d(Option<f64>);

impl Value for CoinPercentChange7d {
    type ValueType = Option<f64>;

    fn value(&self) -> &Self::ValueType {
        &self.0
    }
}

impl TryFrom<f64> for CoinPercentChange7d {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value == f64::default() || value.is_nan() || value.is_infinite() {
            Ok(Self(None))
        } else {
            Ok(Self(Some(value)))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinMarketCap(Option<f64>);

impl Value for CoinMarketCap {
    type ValueType = Option<f64>;

    fn value(&self) -> &Self::ValueType {
        &self.0
    }
}

impl TryFrom<f64> for CoinMarketCap {
    type Error = &'static str;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value == f64::default()
            || value.is_nan()
            || value.is_infinite()
            || value.is_sign_negative()
        {
            Ok(Self(None))
        } else {
            Ok(Self(Some(value)))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinLastUpdated(String);

impl Value for CoinLastUpdated {
    type ValueType = String;

    fn value(&self) -> &Self::ValueType {
        &self.0
    }
}

impl TryFrom<String> for CoinLastUpdated {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err("Coin last updated must not be empty")
        } else {
            Ok(Self(value))
        }
    }
}

pub struct Coin {
    id: CoinId,
    name: CoinName,
    symbol: CoinSymbol,
    price: CoinPrice,
    volume_24h: CoinVolume24h,
    percent_change_1h: CoinPercentChange1h,
    percent_change_24h: CoinPercentChange24h,
    percent_change_7d: CoinPercentChange7d,
    market_cap: CoinMarketCap,
    last_updated: CoinLastUpdated,
}

impl Entity for Coin {}

impl Coin {
    pub fn new(
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
    ) -> Result<Self, &'static str> {
        Ok(Self {
            id: CoinId::try_from(id)?,
            name: CoinName::try_from(name)?,
            symbol: CoinSymbol::try_from(symbol)?,
            price: CoinPrice::try_from(price)?,
            volume_24h: CoinVolume24h::try_from(volume_24h)?,
            percent_change_1h: CoinPercentChange1h::try_from(percent_change_1h)?,
            percent_change_24h: CoinPercentChange24h::try_from(percent_change_24h)?,
            percent_change_7d: CoinPercentChange7d::try_from(percent_change_7d)?,
            market_cap: CoinMarketCap::try_from(market_cap)?,
            last_updated: CoinLastUpdated::try_from(last_updated)?,
        })
    }

    pub fn id(&self) -> &CoinId {
        &self.id
    }

    pub fn name(&self) -> &CoinName {
        &self.name
    }

    pub fn symbol(&self) -> &CoinSymbol {
        &self.symbol
    }

    pub fn price(&self) -> &CoinPrice {
        &self.price
    }

    pub fn volume_24h(&self) -> &CoinVolume24h {
        &self.volume_24h
    }

    pub fn percent_change_1h(&self) -> &CoinPercentChange1h {
        &self.percent_change_1h
    }

    pub fn percent_change_24h(&self) -> &CoinPercentChange24h {
        &self.percent_change_24h
    }

    pub fn percent_change_7d(&self) -> &CoinPercentChange7d {
        &self.percent_change_7d
    }

    pub fn market_cap(&self) -> &CoinMarketCap {
        &self.market_cap
    }

    pub fn last_updated(&self) -> &CoinLastUpdated {
        &self.last_updated
    }
}

impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} - {}({})",
            self.symbol.value(),
            self.name.value(),
            self.id.value()
        )
    }
}

impl From<CreateCoinRequest> for Coin {
    fn from(coin: CreateCoinRequest) -> Self {
        Coin {
            id: CoinId::try_from(coin.id).unwrap(),
            name: CoinName::try_from(coin.name).unwrap(),
            symbol: CoinSymbol::try_from(coin.symbol).unwrap(),
            price: CoinPrice::try_from(coin.price.unwrap_or_default()).unwrap(),
            volume_24h: CoinVolume24h::try_from(coin.volume_24h.unwrap_or_default()).unwrap(),
            percent_change_1h: CoinPercentChange1h::try_from(
                coin.percent_change_1h.unwrap_or_default(),
            )
            .unwrap(),
            percent_change_24h: CoinPercentChange24h::try_from(
                coin.percent_change_24h.unwrap_or_default(),
            )
            .unwrap(),
            percent_change_7d: CoinPercentChange7d::try_from(
                coin.percent_change_7d.unwrap_or_default(),
            )
            .unwrap(),
            market_cap: CoinMarketCap::try_from(coin.market_cap.unwrap_or_default()).unwrap(),
            last_updated: CoinLastUpdated::try_from(coin.last_updated).unwrap(),
        }
    }
}

impl From<CoinResponse> for Coin {
    fn from(coin: CoinResponse) -> Self {
        Coin {
            id: CoinId::try_from(coin.id).unwrap(),
            name: CoinName::try_from(coin.name).unwrap(),
            symbol: CoinSymbol::try_from(coin.symbol).unwrap(),
            price: CoinPrice::try_from(coin.price).unwrap(),
            volume_24h: CoinVolume24h::try_from(coin.volume_24h).unwrap(),
            percent_change_1h: CoinPercentChange1h::try_from(coin.percent_change_1h).unwrap(),
            percent_change_24h: CoinPercentChange24h::try_from(coin.percent_change_24h).unwrap(),
            percent_change_7d: CoinPercentChange7d::try_from(coin.percent_change_7d).unwrap(),
            market_cap: CoinMarketCap::try_from(coin.market_cap).unwrap(),
            last_updated: CoinLastUpdated::try_from(coin.last_updated).unwrap(),
        }
    }
}
