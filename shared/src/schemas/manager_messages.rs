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

pub struct ManagerRequest {
    ip: String,
}

impl ManagerRequest {
    pub fn new(ip: String) -> Self {
        Self { ip }
    }

    pub fn spec_request_json(&self) -> String {
        let request = ManagerRequestSchema::Spec(SpecRequest {
            sender_ip: self.ip.clone(),
        });
        serde_json::to_string(&request).unwrap()
    }
    pub fn usage_overview_request_json(&self) -> String {
        let request = ManagerRequestSchema::UsageOverview(UsageOverviewRequest {
            sender_ip: self.ip.clone(),
        });
        serde_json::to_string(&request).unwrap()
    }
}
