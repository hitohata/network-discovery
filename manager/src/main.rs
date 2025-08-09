mod network;
mod servers;
mod shared_data;

use tracing::{Level, error};

const BROADCAST_ADDRESS: &str = "255.255.255.255";

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(Level::INFO.as_str())
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let mut handlers = vec![];

    // Start the TargetServer in a separate thread
    let target_handler = std::thread::spawn(|| {
        let target_server = shared::server::target_server::TargetServer::new();
        target_server.run().unwrap_or_else(|e| {
            error!("Failed to run TargetServer: {}", e);
        });
    });
    handlers.push(target_handler);

    // Start the DiscoverServer in a separate thread
    let discover_server_handler = std::thread::spawn(|| {
        let discovery_server = crate::servers::discovery_server::DiscoveryServer::new();
        discovery_server.run();
    });
    handlers.push(discover_server_handler);

    for handler in handlers {
        if let Err(e) = handler.join() {
            error!("Thread panicked: {:?}", e);
        }
    }
}
