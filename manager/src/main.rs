mod commands;
mod shared_data;
mod store;
mod threads;

use crate::commands::DiscoveryCommand;
use crate::store::data_store::DataStore;
use tracing::{Level, error};

const BROADCAST_ADDRESS: &str = "255.255.255.255";

#[tokio::main]
async fn main() {
    // set up tracing for logging
    tracing_subscriber::fmt()
        .with_env_filter(Level::INFO.as_str())
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    // List to hold thread handlers
    let mut handlers = vec![];

    // channel for commands
    let (command_tx, command_rx): (
        tokio::sync::mpsc::Sender<DiscoveryCommand>,
        tokio::sync::mpsc::Receiver<DiscoveryCommand>,
    ) = tokio::sync::mpsc::channel(32);
    // channel for data
    let (response_tx, response_rx): (
        tokio::sync::broadcast::Sender<shared::schemas::target_messages::ResponseSchema>,
        tokio::sync::broadcast::Receiver<shared::schemas::target_messages::ResponseSchema>,
    ) = tokio::sync::broadcast::channel(32);

    let data_store_command_tx = command_tx.clone();
    let data_store = std::sync::Arc::new(tokio::sync::Mutex::new(DataStore::new()));

    // Start the TargetServer in a separate thread
    let target_handler = tokio::spawn(async {
        let target_server = shared::server::target_server::TargetServer::new();
        target_server.run().await.unwrap_or_else(|e| {
            error!("Failed to run TargetServer: {}", e);
        });
    });
    handlers.push(target_handler);

    // Start the DiscoverServer in a separate thread
    let discovery_server = crate::threads::discovery_server::DiscoveryServer::new();
    let discover_server_handler =
        tokio::spawn(async move { discovery_server.run(command_rx, response_tx.clone()).await });
    handlers.push(discover_server_handler);

    // run data store service
    let data_store_service =
        crate::threads::data_store_service::DataStoreService::new(data_store.clone());
    data_store_service.run(data_store_command_tx, response_rx);

    for handler in handlers {
        if let Err(e) = handler.await {
            error!("Thread panicked: {:?}", e);
        }
    }
}
