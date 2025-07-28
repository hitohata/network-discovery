use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
enum MessageType {
    #[serde(rename = "discovery_request")]
    DiscoveryRequest,
    #[serde(rename = "discovery_response")]
    DiscoveryResponse,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DiscoveryRequest {
    #[serde(rename = "type")]
    msg_type: MessageType,
    sender_ip: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct DiscoveryResponse {
    #[serde(rename = "type")]
    msg_type: MessageType,
    hostname: String,
    ip_address: String,
    cpu_usage: Vec<f32> ,
    memory_total_mb: u64,
    memory_used_mb: u64,
    cpu_temperature: f32,
}