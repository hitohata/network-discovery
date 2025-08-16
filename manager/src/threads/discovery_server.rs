use crate::BROADCAST_ADDRESS;
use crate::commands::DiscoveryCommand;
use shared::schemas::target_messages::ResponseSchema;
use shared::utils::tools::get_ip;
use std::net::UdpSocket;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use tracing::{debug, error, info};

pub struct DiscoveryServer {}

impl DiscoveryServer {
    pub fn new() -> Self {
        Self {}
    }
    pub fn run(&self, command_rx: Receiver<DiscoveryCommand>, response_tx: Sender<ResponseSchema>) {
        info!("Starting Manager...");

        let ip = get_ip();

        let request = Arc::new(shared::schemas::manager_messages::ManagerRequest::new(
            ip.to_string(),
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
                        DiscoveryCommand::DeviceInformation(target_ip) => {
                            debug!("Device Info Request: {:?}", target_ip);
                            if let Err(e) = command_socket.send_to(
                                spec_request.as_bytes(),
                                format!("{}:{}", target_ip, shared::utils::constants::TARGET_PORT),
                            ) {
                                command_socket
                                    .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                                    .expect("Failed to set read timeout");
                                error!("Failed to send Spec request: {}", e);
                                continue;
                            }
                        }
                    },
                    Err(e) => {
                        error!("Failed to receive command: {}", e);
                        continue;
                    }
                }
            }
        });

        let usage_socket = socket.clone();
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
                            if let Err(e) = response_tx.send(ResponseSchema::Spec(spec)) {
                                error!("Failed to send Spec response: {}", e);
                            }
                        }
                        shared::schemas::target_messages::ResponseSchema::UsageOverview(usage) => {
                            if let Err(e) = response_tx.send(ResponseSchema::UsageOverview(usage)) {
                                error!("Failed to send Usage response: {}", e);
                            }
                        }
                    }
                }
                std::thread::sleep(std::time::Duration::from_secs(5));
            }
        });
    }
}
