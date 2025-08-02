use serde_json;
use shared::scan::usage;
use shared::schemas;
use std::net::UdpSocket;
use tracing::{error, info, Level};
use tracing_subscriber;

fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(Level::INFO.as_str())
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let socket = UdpSocket::bind(format!("0.0.0.0:{}", shared::utils::constants::TARGET_PORT))?;
    socket.set_broadcast(true)?;
    info!("Starting HTTP server on {}", socket.local_addr()?);

    let system = usage::SystemInfo::default();

    let mut buf = [0; 1024];

    loop {
        let (amt, src) = socket.recv_from(&mut buf)?;
        let received_data = &buf[..amt];

        let Ok(request) = serde_json::from_slice::<schemas::manager_messages::ManagerRequestSchema>(
            received_data,
        ) else {
            error!(
                "Failed to parse received data from {}: {:?}",
                src,
                String::from_utf8_lossy(received_data)
            );
            continue;
        };

        match request {
            schemas::manager_messages::ManagerRequestSchema::Spec(req) => {
                info!("Received Spec request from {}: {:?}", src, req);
                let response = schemas::target_messages::SpecResponse::new(
                    system.get_machine_info().to_owned(),
                );
                let Ok(response_data) = serde_json::to_vec(&response) else {
                    error!("Failed to serialize response for Spec request from {}", src);
                    continue;
                };
                socket.send_to(&response_data, src)?;
            }
            schemas::manager_messages::ManagerRequestSchema::UsageOverview(req) => {
                info!("Received Usage Overview request from {}: {:?}", src, req);
                let response =
                    schemas::target_messages::UsageOverviewResponse::new(system.get_usage());
                let Ok(response_data) = serde_json::to_vec(&response) else {
                    error!(
                        "Failed to serialize response for Usage Overview request from {}",
                        src
                    );
                    continue;
                };
                socket.send_to(&response_data, src)?;
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
