mod network;
mod servers;
mod shared_data;

use crate::network::search_networks;
use std::env;
use std::net::UdpSocket;
use tracing::{Level, error, info};

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(Level::INFO.as_str())
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    info!("Starting Manager...");

    let args: Vec<String> = env::args().collect();
    let ip = if args.len() < 2 {
        match search_networks() {
            Some(ip) => ip.to_string(),
            None => {
                panic!("No valid IP address found.");
            }
        }
    } else {
        args[1].to_string()
    };

    let request = shared::schemas::manager_messages::ManagerRequest::new(ip.clone());

    let socket = UdpSocket::bind(format!("{}:{}", ip, shared::utils::constants::TARGET_PORT))
        .expect("Failed to bind UDP socket");
    socket.set_broadcast(true).unwrap_or_else(|err| {
        panic!("Failed to set broadcast UDP broadcast: {err}");
    });

    loop {
        let spec_request = request.spec_request_json();
        let usage_request = request.usage_overview_request_json();

        socket
            .send_to(
                spec_request.as_bytes(),
                format!(
                    "{}:{}",
                    "255.255.255.255",
                    shared::utils::constants::TARGET_PORT
                ),
            )
            .expect("Failed to send request"); // todo fix
        socket
            .send_to(
                usage_request.as_bytes(),
                format!(
                    "{}:{}",
                    "255.255.255.255",
                    shared::utils::constants::TARGET_PORT
                ),
            )
            .expect("Failed to send request"); // todo fix

        socket
            .set_read_timeout(Some(std::time::Duration::from_secs(3)))
            .expect("Failed to set read timeout"); // todo

        let mut buf = [0; 1024];

        loop {
            let (amt, src) = match socket.recv_from(&mut buf) {
                Ok(result) => result,
                Err(e) => {
                    error!("Failed to receive data from socket: {}. Retrying...", e);
                    break;
                }
            };

            let received_data = &buf[..amt];

            let Ok(received_data) = serde_json::from_slice::<
                shared::schemas::target_messages::ResponseSchema,
            >(received_data) else {
                error!(
                    "Failed to parse received data from {}: {:?}",
                    src,
                    String::from_utf8_lossy(received_data)
                );
                info!(
                    "Received data: {:?}",
                    String::from_utf8_lossy(received_data)
                );
                continue;
            };

            match received_data {
                shared::schemas::target_messages::ResponseSchema::Spec(spec) => {
                    info!("Received Spec response from {}: {:?}", src, spec);
                }
                shared::schemas::target_messages::ResponseSchema::UsageOverview(usage) => {
                    info!("Received Usage Overview response from {}: {:?}", src, usage);
                }
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
