use crate::schemas::device_info::{MachineInfo, MachineUsage};
use serde::{Deserialize, Serialize};
use serde_json::json;

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
    spec: MachineInfo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct UsageOverviewResponse {
    usage: MachineUsage,
}

impl SpecResponse {
    pub fn spec_response_json(spec: MachineInfo) -> String {
        let response = ResponseSchema::Spec(SpecResponse { spec });
        json!(response).to_string()
    }
}

impl UsageOverviewResponse {
    pub fn usage_overview_response_json(usage: MachineUsage) -> String {
        let response = ResponseSchema::UsageOverview(UsageOverviewResponse { usage });
        json!(response).to_string()
    }
}
