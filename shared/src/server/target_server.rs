//! Server-side code for the target device.
//!
//! This module defines the `TargetServer` struct, which listens for UDP requests from the manager,
//! processes requests for system information and usage overview, and sends appropriate responses.

use crate::scan::usage;
use crate::schemas;
use crate::utils::tools::get_ip;
use std::net::UdpSocket;
use tracing::{debug, error, info};

pub struct TargetServer {
    system_info: usage::SystemInfo,
}

impl TargetServer {
    pub fn new() -> Self {
        let system_info = usage::SystemInfo::default();
        Self { system_info }
    }

    pub fn run(&self) -> std::io::Result<()> {
        let ip = get_ip();
        let socket = UdpSocket::bind(format!(
            "{}:{}",
            "0.0.0.0",
            crate::utils::constants::TARGET_PORT
        ))?;
        socket.set_broadcast(true)?;
        info!("Starting UDP server on {}", socket.local_addr()?);

        let mut buf = [0; 1024];

        loop {
            let (amt, src) = socket.recv_from(&mut buf)?;
            let received_data = &buf[..amt];

            let Ok(request) = serde_json::from_slice::<
                schemas::manager_messages::ManagerRequestSchema,
            >(received_data) else {
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
                    let response = schemas::target_messages::SpecResponse::spec_response_json(
                        ip,
                        self.system_info.get_machine_info().to_owned(),
                    );
                    debug!("Spec response: {:?}", response);
                    socket.send_to(response.as_bytes(), src)?;
                }
                schemas::manager_messages::ManagerRequestSchema::UsageOverview(req) => {
                    info!("Received Usage Overview request from {}: {:?}", src, req);
                    let response = schemas::target_messages::UsageOverviewResponse::usage_overview_response_json(
                        ip,
                        self.system_info.get_usage().to_owned(),
                    );
                    debug!("usage response: {:?}", response);
                    socket.send_to(response.as_bytes(), src)?;
                }
            }
        }
    }
}

impl Default for TargetServer {
    fn default() -> Self {
        Self::new()
    }
}
