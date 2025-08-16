use crate::schemas::device_info::{MachineInfo, MachineUsage};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::net::Ipv4Addr;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "response", rename_all = "camelCase")]
pub enum ResponseSchema {
    #[serde(rename = "spec")]
    Spec(SpecResponse),
    #[serde(rename = "usageOverview")]
    UsageOverview(UsageOverviewResponse),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SpecResponse {
    pub ip: Ipv4Addr,
    pub spec: MachineInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UsageOverviewResponse {
    pub ip: Ipv4Addr,
    pub usage: MachineUsage,
}

impl SpecResponse {
    pub fn spec_response_json(ip: Ipv4Addr, spec: MachineInfo) -> String {
        let response = ResponseSchema::Spec(SpecResponse { ip, spec });
        json!(response).to_string()
    }
}

impl UsageOverviewResponse {
    pub fn usage_overview_response_json(ip: Ipv4Addr, usage: MachineUsage) -> String {
        let response = ResponseSchema::UsageOverview(UsageOverviewResponse { ip, usage });
        json!(response).to_string()
    }
}
