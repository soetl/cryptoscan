use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tauri::State;
use validator::Validate;

use crate::{domain::{self, settings::settings::Setting, Value}, AppState};

use super::errors::TauriErrors;

#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct CreateSettingRequest {
    #[validate(length(min = 1))]
    pub key: String,
    #[validate(length(min = 1))]
    pub value: String,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct SettingResponse {
    pub key: String,
    pub value: String,
}

impl From<Setting> for SettingResponse {
    fn from(setting: Setting) -> Self {
        Self {
            key: setting.key().value().to_string(),
            value: setting.value().value().to_string(),
        }
    }
}

#[tauri::command]
pub async fn create_setting(request: CreateSettingRequest, state: State<'_, Arc<AppState>>,) -> Result<String, TauriErrors> {
    request.validate()?;

    let result = domain::settings::create_setting::create_setting(
        state.sqlite_repo.clone(),
        request,
    ).await;

    match result {
        Ok(setting) => Ok(serde_json::to_string(&SettingResponse::from(setting)).unwrap()),
        Err(e) => Err(TauriErrors::UnknownError(e.to_string())),
    }
}

#[tauri::command]
pub async fn find_setting(request: String, state: State<'_, Arc<AppState>>) -> Result<String, TauriErrors> {
    let result = domain::settings::find_setting::find_setting(state.sqlite_repo.clone(), request).await;

    match result {
        Ok(setting) => Ok(serde_json::to_string(&SettingResponse::from(setting)).unwrap()),
        Err(e) => Err(TauriErrors::UnknownError(e.to_string())),
    }
}