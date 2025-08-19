use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MachineInfo {
    pub os: String,
    pub os_version: String,
    pub host_name: String,
    pub kernel_version: String,
    pub number_of_cpu: usize,
    pub arch: String,
    pub brand: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MachineUsage {
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub cpu_usage: Vec<f32>,
    pub cpu_frequency: Vec<u64>,
    pub network_down: u64,
    pub network_up: u64,
}
