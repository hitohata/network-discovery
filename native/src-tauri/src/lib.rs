mod commands;

use crate::commands::{get_node_detail, get_nodes};
use shared::store::data_store::{DataStore, DataStoreType};
use tauri::Manager;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {name}")
}

pub(crate) struct AppState {
    #[cfg(not(mobile))]
    pub data_store: DataStoreType,
}

#[cfg(mobile)]
#[tauri::mobile_entry_point]
fn main() {
    build_app()
}

#[cfg(not(mobile))]
#[tokio::main]
pub async fn run() {
    build_app()
}

fn build_app() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, get_node_detail, get_nodes])
        .setup(|app| {
            // run thread when it is not mobile
            #[cfg(not(mobile))]
            {
                let data_store = DataStore::init();

                let manager_server = shared::server::manager_server::ManagerServer::new(
                    std::sync::Arc::clone(&data_store),
                );

                app.manage(AppState {
                    data_store: std::sync::Arc::clone(&data_store),
                });

                tauri::async_runtime::spawn(async move {
                    manager_server.run().await;
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
