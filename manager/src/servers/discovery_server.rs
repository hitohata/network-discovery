use crate::BROADCAST_ADDRESS;
use crate::commands::DiscoveryCommand;
use crate::network::search_networks;
use std::env;
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use tracing::{error, info};

pub(crate) enum Command {}

pub struct DiscoveryServer {
    command_tx: Sender<DiscoveryCommand>,
}

impl DiscoveryServer {
    pub fn run() -> Self {
        info!("Starting Manager...");

        let (command_tx, command_rx): (Sender<DiscoveryCommand>, Receiver<DiscoveryCommand>) =
            std::sync::mpsc::channel();
        let (response_tx, response_rx): (
            Sender<shared::schemas::target_messages::ResponseSchema>,
            Receiver<shared::schemas::target_messages::ResponseSchema>,
        ) = std::sync::mpsc::channel();

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

        let request = Arc::new(shared::schemas::manager_messages::ManagerRequest::new(
            ip.clone(),
        ));

        let socket = Arc::new(
            UdpSocket::bind(format!("{}:{}", ip, shared::utils::constants::HOST_PORT))
                .expect("Failed to bind UDP socket"),
        );
        socket.set_broadcast(true).unwrap_or_else(|err| {
            panic!("Failed to set broadcast UDP broadcast: {err}");
        });

        let r = request.clone();

        let command_socket = socket.clone();
        let command_request = request.clone();
        std::thread::spawn(move || {
            let spec_request = command_request.spec_request_json();
            loop {
                match command_rx.recv() {
                    Ok(command) => match command {
                        DiscoveryCommand::DeviceUsage => {
                            break;
                        }
                        DiscoveryCommand::DeviceInformation(_ip) => {
                            if let Err(e) = command_socket.send_to(
                                spec_request.as_bytes(),
                                format!(
                                    "{}:{}",
                                    BROADCAST_ADDRESS,
                                    shared::utils::constants::TARGET_PORT
                                ),
                            ) {
                                command_socket
                                    .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                                    .expect("Failed to set read timeout");
                                error!("Failed to send Spec request: {}", e);
                                break;
                            }
                        }
                    },
                    Err(e) => {
                        error!("Failed to receive command: {}", e);
                        break;
                    }
                }
            }
        });

        let usage_socket = socket.clone();
        let usage_request = request.clone();
        std::thread::spawn(move || {
            let usage_request = r.usage_overview_request_json();
            loop {
                if let Err(e) = usage_socket.send_to(
                    usage_request.as_bytes(),
                    format!(
                        "{}:{}",
                        BROADCAST_ADDRESS,
                        shared::utils::constants::TARGET_PORT
                    ),
                ) {
                    usage_socket
                        .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                        .expect("Failed to set read timeout");
                    error!("Failed to send Spec request: {}", e);
                    break;
                }
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        });

        std::thread::spawn(move || {
            loop {
                Arc::clone(&socket)
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
        });
        Self { command_tx }
    }
}
