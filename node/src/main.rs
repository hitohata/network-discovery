use tracing::Level;

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(Level::INFO.as_str())
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let target_server = shared::server::target_server::TargetServer::new();

    target_server.run().map_err(|e| {
        tracing::error!("Failed to run TargetServer: {}", e);
        e
    })
}
