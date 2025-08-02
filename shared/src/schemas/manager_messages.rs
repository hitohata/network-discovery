use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "request", rename_all = "camelCase")]
pub enum ManagerRequestSchema {
    #[serde(rename = "spec")]
    Spec(SpecRequest),
    #[serde(rename = "usageOverview")]
    UsageOverview(UsageOverviewRequest),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpecRequest {
    sender_ip: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UsageOverviewRequest {
    sender_ip: String,
}
