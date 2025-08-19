//! The data store of nodes.

use crate::schemas::device_info::{MachineInfo, MachineUsage};
use serde::Serialize;
use std::net::Ipv4Addr;

struct MachineUsageRecord {
    machine_usage: MachineUsage,
    // Unix timestamp in seconds
    timestamp: u64,
}

struct Node {
    ip: Ipv4Addr,
    machine_info: Option<MachineInfo>,
    usage: std::collections::VecDeque<MachineUsageRecord>,
    last_updated: std::time::SystemTime,
}

impl Node {
    fn new(ip: Ipv4Addr, machine_info: Option<MachineInfo>, machine_usage: MachineUsage) -> Self {
        let mut usage = std::collections::VecDeque::with_capacity(5 * 100);
        let machine_usage = MachineUsageRecord {
            machine_usage,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        usage.push_front(machine_usage);
        Self {
            ip,
            machine_info,
            usage,
            last_updated: std::time::SystemTime::now(),
        }
    }

    /// remove the first element from the usage queue, then push the new usage to the end
    fn update_usage(&mut self, machine_usage: MachineUsage) {
        if self.usage.len() >= 500 {
            self.usage.pop_back();
        }
        let machine_usage = MachineUsageRecord {
            machine_usage,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        };
        self.usage.push_front(machine_usage);
        self.last_updated = std::time::SystemTime::now();
    }

    /// update the machine info
    fn update_info(&mut self, machine_info: MachineInfo) {
        // if the machine info is different, reset the usage queue
        if let Some(info) = &self.machine_info
            && info != &machine_info
        {
            self.usage = std::collections::VecDeque::with_capacity(5 * 100);
        }
        self.machine_info = Some(machine_info);
    }

    fn to_node_data(&self) -> NodeData {
        NodeData {
            ip: self.ip,
            machine_info: self.machine_info.clone(),
            usage: self
                .usage
                .iter()
                .map(|record| MachineUsageData {
                    machine_usage: record.machine_usage.clone(),
                    timestamp: record.timestamp,
                })
                .collect(),
            last_updated: self.last_updated,
        }
    }

    fn to_overview(&self) -> NodeOverview {
        NodeOverview {
            ip: self.ip,
            machine_info: self.machine_info.clone(),
            usage: self
                .usage
                .front()
                .map(|record| record.machine_usage.clone()),
            last_updated: self.last_updated,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MachineUsageData {
    pub machine_usage: MachineUsage,
    // Unix timestamp in seconds
    pub timestamp: u64,
}

/// The DTO of node data.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeData {
    pub ip: Ipv4Addr,
    pub machine_info: Option<MachineInfo>,
    pub usage: Vec<MachineUsageData>,
    pub last_updated: std::time::SystemTime,
}

/// The overview of a node.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeOverview {
    pub ip: Ipv4Addr,
    pub machine_info: Option<MachineInfo>,
    pub usage: Option<MachineUsage>,
    pub last_updated: std::time::SystemTime,
}

pub type DataStoreType = std::sync::Arc<tokio::sync::RwLock<DataStore>>;

pub struct DataStore {
    nodes: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<Ipv4Addr, Node>>>,
}

impl DataStore {
    pub fn new() -> Self {
        Self {
            nodes: std::sync::Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }
    /// This method returns `DataStore` with Arc<RwLock<DataStore>>
    pub fn init() -> DataStoreType {
        std::sync::Arc::new(tokio::sync::RwLock::new(Self {
            nodes: std::sync::Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
        }))
    }

    /// get nodes
    pub fn get_node_status(&self) -> std::vec::Vec<NodeOverview> {
        let node_lock = self.nodes.read().unwrap();
        node_lock
            .values()
            .map(|node| node.to_overview())
            .collect::<std::vec::Vec<NodeOverview>>()
    }

    pub fn get_node(&self, ip: Ipv4Addr) -> Option<NodeData> {
        let node_lock = self.nodes.read().unwrap();
        node_lock.get(&ip).map(|node| node.to_node_data())
    }

    pub fn get_node_overview(&self) -> std::vec::Vec<NodeOverview> {
        let node_lock = self.nodes.read().unwrap();
        node_lock
            .values()
            .map(|node| node.to_overview())
            .collect::<std::vec::Vec<NodeOverview>>()
    }

    /// Add or update a node's data
    pub fn update_usage(&mut self, ip: Ipv4Addr, machine_usage: MachineUsage) {
        let mut node_lock = self.nodes.write().unwrap();

        node_lock
            .entry(ip)
            .and_modify(|node| node.update_usage(machine_usage.clone()))
            .or_insert_with(|| Node::new(ip, None, machine_usage));
    }

    /// Add the machine info to the node
    /// If there is no node with the given IP, do nothing
    pub fn update_node_information(&mut self, ip: Ipv4Addr, machine_info: MachineInfo) {
        let mut node_lock = self.nodes.write().unwrap();

        if let Some(node) = node_lock.get_mut(&ip) {
            node.update_info(machine_info);
        }
    }

    /// Remove a node from the data store
    pub fn remove_node(&mut self, ip: &Ipv4Addr) {
        let mut node_lock = self.nodes.write().unwrap();
        node_lock.remove(ip);
    }
}
impl Default for DataStore {
    fn default() -> Self {
        Self::new()
    }
}
