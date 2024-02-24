use std::sync::Arc;

use config::Config;
use driven::repository::sqlite::SqliteRepository;
use tauri::{async_runtime::Mutex, Manager, Wry};
use tauri_plugin_store::{Store, StoreBuilder};

mod config;
mod domain;
mod driven;
mod driving;
mod fetch;

struct AppState {
    config: config::Config,
    store: Mutex<Store<Wry>>,
    sqlite_repo: Arc<Mutex<SqliteRepository>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let config = Config::from(String::from(app.path().app_data_dir()?.to_str().unwrap()));
            let store = Mutex::new(
                StoreBuilder::new(app.path().app_data_dir()?).build(app.handle().clone()),
            );
            let sqlite_repo = Arc::new(Mutex::new(SqliteRepository::new(&config.sqlite)));

            let state = AppState {
                config,
                store,
                sqlite_repo,
            };

            app.manage(Arc::new(state));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            driving::tauri::coins::create_coin,
            driving::tauri::coins::create_coins,
            driving::tauri::coins::update_coins,
            driving::tauri::coins::find_coin,
            driving::tauri::coins::find_coins,
            driving::tauri::coins::delete_coin,
            driving::tauri::coins::delete_all_coins,
            driving::tauri::coins::get_all_coins,
            driving::tauri::coins::fetch_coins_by_id,
            driving::tauri::coins::fetch_coins_by_symbol,
            driving::tauri::coins::set_cmc_token,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
