mod commands;
mod shared_data;
mod store;
mod threads;

use crate::commands::DiscoveryCommand;
use crate::store::data_store::DataStore;
use std::sync::mpsc::{Receiver, Sender};
use tracing::{Level, error};

const BROADCAST_ADDRESS: &str = "255.255.255.255";

fn main() {
    // set up tracing for logging
    tracing_subscriber::fmt()
        .with_env_filter(Level::INFO.as_str())
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    // List to hold thread handlers
    let mut handlers = vec![];

    // channel for commands
    let (command_tx, command_rx): (Sender<DiscoveryCommand>, Receiver<DiscoveryCommand>) =
        std::sync::mpsc::channel();
    let command_tx = std::sync::Arc::new(command_tx);
    // channel for data
    let (response_tx, response_rx): (
        Sender<shared::schemas::target_messages::ResponseSchema>,
        Receiver<shared::schemas::target_messages::ResponseSchema>,
    ) = std::sync::mpsc::channel();

    let data_store_command_tx = command_tx.clone();
    let data_store =
        std::sync::Arc::new(std::sync::Mutex::new(DataStore::new(data_store_command_tx)));

    // Start the TargetServer in a separate thread
    let target_handler = std::thread::spawn(|| {
        let target_server = shared::server::target_server::TargetServer::new();
        target_server.run().unwrap_or_else(|e| {
            error!("Failed to run TargetServer: {}", e);
        });
    });
    handlers.push(target_handler);

    // Start the DiscoverServer in a separate thread
    let discovery_server = crate::threads::discovery_server::DiscoveryServer::new();
    let discover_server_handler =
        std::thread::spawn(move || discovery_server.run(command_rx, response_tx.clone()));
    handlers.push(discover_server_handler);

    // run data store service
    let data_store_service =
        crate::threads::data_store_service::DataStoreService::new(data_store.clone());
    data_store_service.run(response_rx);

    for handler in handlers {
        if let Err(e) = handler.join() {
            error!("Thread panicked: {:?}", e);
        }
    }
}
