use shared::store::data_store::DataStore;
use tracing::Level;

#[tokio::main]
async fn main() {
    // set up tracing for logging
    tracing_subscriber::fmt()
        .with_env_filter(Level::INFO.as_str())
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let data_store = DataStore::init();

    let mut handlers = vec![];

    // start the manager server
    let data_store_for_server = data_store.clone();
    let manager_server = shared::server::manager_server::ManagerServer::new(data_store_for_server);
    let manager_server_handler = tokio::spawn(async move {
        manager_server.run().await;
    });
    handlers.push(manager_server_handler);
}
