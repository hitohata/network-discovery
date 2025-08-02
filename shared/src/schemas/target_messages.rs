use crate::schemas::device_info::{MachineInfo, MachineUsage};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "request", rename_all = "camelCase")]
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
    pub fn new(spec: MachineInfo) -> Self {
        Self { spec }
    }
}

impl UsageOverviewResponse {
    pub fn new(usage: MachineUsage) -> Self {
        Self { usage }
    }
}
