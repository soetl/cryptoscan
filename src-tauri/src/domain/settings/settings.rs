use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{domain::{Entity, Value}, driving::tauri::settings::CreateSettingRequest};

#[derive(Error, Debug, Serialize)]
pub enum SettingError {
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Unknown error: {0}")]
    Unknown(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SettingKey(String);

impl Value for SettingKey {
    type ValueType = String;

    fn value(&self) -> &Self::ValueType {
        &self.0
    }
}

impl TryFrom<String> for SettingKey {
    type Error = SettingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(SettingError::InvalidData("Setting key must not be empty".to_string()))
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SettingValue(String);

impl Value for SettingValue {
    type ValueType = String;

    fn value(&self) -> &Self::ValueType {
        &self.0
    }
}

impl TryFrom<String> for SettingValue {
    type Error = SettingError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            Err(SettingError::InvalidData("Setting value must not be empty".to_string()))
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Debug, Clone)]
pub struct Setting {
    key: SettingKey,
    value: SettingValue,
}

impl Entity for Setting {}

impl Setting {
    pub fn new(key: String, value: String) -> Result<Self, SettingError> {
        Ok(Self {
            key: SettingKey::try_from(key)?,
            value: SettingValue::try_from(value)?,
        })
    }

    pub fn key(&self) -> &SettingKey {
        &self.key
    }

    pub fn value(&self) -> &SettingValue {
        &self.value
    }
}

impl TryFrom<CreateSettingRequest> for Setting {
    type Error = SettingError;

    fn try_from(value: CreateSettingRequest) -> Result<Self, Self::Error> {
        Self::new(value.key, value.value)
    }
}
