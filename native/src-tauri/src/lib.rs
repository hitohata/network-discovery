use shared::store::data_store::DataStore;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() {

    let data_store = DataStore::init();
    let data_store_for_server = data_store.clone();
    let manager_server = shared::server::manager_server::ManagerServer::new(data_store_for_server);
    tokio::spawn(async move {
        manager_server.run().await;
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
