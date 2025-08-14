mod commands;
mod network;
mod servers;
mod shared_data;
mod store;

use crate::commands::DiscoveryCommand;
use std::net::Ipv4Addr;
use std::sync::mpsc::{Receiver, Sender};
use tracing::{Level, error, info};

const BROADCAST_ADDRESS: &str = "255.255.255.255";

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(Level::INFO.as_str())
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let mut handlers = vec![];

    // channel for commands
    let (command_tx, command_rx): (Sender<DiscoveryCommand>, Receiver<DiscoveryCommand>) =
        std::sync::mpsc::channel();
    // channel for data
    let (response_tx, response_rx): (
        Sender<shared::schemas::target_messages::ResponseSchema>,
        Receiver<shared::schemas::target_messages::ResponseSchema>,
    ) = std::sync::mpsc::channel();

    // Start the TargetServer in a separate thread
    let target_handler = std::thread::spawn(|| {
        let target_server = shared::server::target_server::TargetServer::new();
        target_server.run().unwrap_or_else(|e| {
            error!("Failed to run TargetServer: {}", e);
        });
    });
    handlers.push(target_handler);

    // Start the DiscoverServer in a separate thread
    let discovery_server = crate::servers::discovery_server::DiscoveryServer::new();
    let discover_server_handler =
        std::thread::spawn(move || discovery_server.run(command_rx, response_tx.clone()));
    handlers.push(discover_server_handler);

    let command_handler = std::thread::spawn(move || {
        loop {
            command_tx
                .send(DiscoveryCommand::DeviceInformation(Ipv4Addr::new(
                    255, 255, 255, 255,
                )))
                .unwrap_or_else(|e| {
                    error!("Failed to send command: {}", e);
                });
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    });
    handlers.push(command_handler);

    loop {
        match response_rx.recv() {
            Ok(response) => match response {
                shared::schemas::target_messages::ResponseSchema::Spec(resp) => {
                    info!("Received Spec response: {:?}", resp);
                }
                shared::schemas::target_messages::ResponseSchema::UsageOverview(resp) => {
                    info!("Received Usage Overview response: {:?}", resp);
                }
            },
            Err(e) => {
                error!("Failed to receive response: {}", e);
            }
        }
    }

    // TODO: remove commented code below
    // for handler in handlers {
    //     if let Err(e) = handler.join() {
    //         error!("Thread panicked: {:?}", e);
    //     }
    // }
}
