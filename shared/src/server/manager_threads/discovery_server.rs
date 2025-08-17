use crate::commands::DiscoveryCommand;
use crate::schemas::target_messages::ResponseSchema;
use crate::utils::tools::get_ip;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tracing::{debug, error, info};

const BROADCAST_ADDRESS: &str = "255.255.255.255";

pub struct DiscoveryServer {}

impl DiscoveryServer {
    pub fn new() -> Self {
        Self {}
    }
    pub async fn run(
        &self,
        mut command_rx: tokio::sync::mpsc::Receiver<DiscoveryCommand>,
        response_tx: tokio::sync::broadcast::Sender<ResponseSchema>,
    ) {
        info!("Starting Manager...");

        let ip = get_ip();

        let request = Arc::new(crate::schemas::manager_messages::ManagerRequest::new(
            ip.to_string(),
        ));

        let socket = Arc::new(
            UdpSocket::bind(format!("{}:{}", ip, crate::utils::constants::HOST_PORT))
                .await
                .expect("Failed to bind UDP socket"),
        );
        socket.set_broadcast(true).unwrap_or_else(|err| {
            panic!("Failed to set broadcast UDP broadcast: {err}");
        });

        let r = request.clone();

        let command_socket = socket.clone();
        let command_request = request.clone();
        tokio::task::spawn(async move {
            let spec_request = command_request.spec_request_json();
            loop {
                match command_rx.recv().await {
                    Some(command) => match command {
                        DiscoveryCommand::DeviceInformation(target_ip) => {
                            debug!("Device Info Request: {:?}", target_ip);
                            if let Err(e) = command_socket
                                .send_to(
                                    spec_request.as_bytes(),
                                    format!(
                                        "{}:{}",
                                        target_ip,
                                        crate::utils::constants::TARGET_PORT
                                    ),
                                )
                                .await
                            {
                                error!("Failed to send Spec request: {}", e);
                                continue;
                            }
                        }
                    },
                    None => {
                        error!("Command channel closed, exiting command thread.");
                        continue;
                    }
                }
            }
        });

        let usage_socket = socket.clone();
        tokio::spawn(async move {
            let usage_request = r.usage_overview_request_json();
            loop {
                if let Err(e) = usage_socket
                    .send_to(
                        usage_request.as_bytes(),
                        format!(
                            "{}:{}",
                            BROADCAST_ADDRESS,
                            crate::utils::constants::TARGET_PORT
                        ),
                    )
                    .await
                {
                    error!("Failed to send Spec request: {}", e);
                    break;
                }
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        });

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let (amt, src) = match socket.recv_from(&mut buf).await {
                    Ok(result) => result,
                    Err(e) => {
                        error!("Failed to receive data from socket: {}. Retrying...", e);
                        continue;
                    }
                };

                let received_data = &buf[..amt];

                let Ok(received_data) = serde_json::from_slice::<
                    crate::schemas::target_messages::ResponseSchema,
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
                    crate::schemas::target_messages::ResponseSchema::Spec(spec) => {
                        if let Err(e) = response_tx.send(ResponseSchema::Spec(spec)) {
                            error!("Failed to send Spec response: {}", e);
                        }
                    }
                    crate::schemas::target_messages::ResponseSchema::UsageOverview(usage) => {
                        if let Err(e) = response_tx.send(ResponseSchema::UsageOverview(usage)) {
                            error!("Failed to send Usage response: {}", e);
                        }
                    }
                }
            }
        });
    }
}
