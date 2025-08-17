//! The data store of nodes.

use shared::schemas::device_info::{MachineInfo, MachineUsage};
use std::net::Ipv4Addr;

struct Node {
    ip: Ipv4Addr,
    machine_info: Option<MachineInfo>,
    usage: std::collections::VecDeque<MachineUsage>,
    last_updated: std::time::SystemTime,
}

impl Node {
    fn new(ip: Ipv4Addr, machine_info: Option<MachineInfo>, machine_usage: MachineUsage) -> Self {
        let mut usage = std::collections::VecDeque::with_capacity(5 * 100);
        usage.push_back(machine_usage);
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
            self.usage.pop_front();
        }
        self.usage.push_back(machine_usage);
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
            usage: self.usage.clone(),
            last_updated: self.last_updated,
        }
    }
}

/// The DTO of node data.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct NodeData {
    pub ip: Ipv4Addr,
    pub machine_info: Option<MachineInfo>,
    pub usage: std::collections::VecDeque<MachineUsage>,
    pub last_updated: std::time::SystemTime,
}

pub struct DataStore {
    nodes: std::sync::Arc<std::sync::RwLock<std::collections::HashMap<Ipv4Addr, Node>>>,
}

impl DataStore {
    pub(crate) fn new() -> Self {
        Self {
            nodes: std::sync::Arc::new(std::sync::RwLock::new(std::collections::HashMap::new())),
        }
    }

    /// get nodes
    pub(crate) fn get_nodes(&self) -> std::vec::Vec<NodeData> {
        let node_lock = self.nodes.read().unwrap();
        node_lock
            .values()
            .map(|node| node.to_node_data())
            .collect::<std::vec::Vec<NodeData>>()
    }

    #[allow(dead_code)]
    pub(crate) fn get_node(&self, ip: Ipv4Addr) -> Option<NodeData> {
        let node_lock = self.nodes.read().unwrap();
        node_lock.get(&ip).map(|node| node.to_node_data())
    }

    /// Add or update a node's data
    pub(crate) fn update_usage(&mut self, ip: Ipv4Addr, machine_usage: MachineUsage) {
        let mut node_lock = self.nodes.write().unwrap();

        node_lock
            .entry(ip)
            .and_modify(|node| node.update_usage(machine_usage.clone()))
            .or_insert_with(|| Node::new(ip, None, machine_usage));
    }

    /// Add the machine info to the node
    /// If there is no node with the given IP, do nothing
    pub(crate) fn update_node_information(&mut self, ip: Ipv4Addr, machine_info: MachineInfo) {
        let mut node_lock = self.nodes.write().unwrap();

        if let Some(node) = node_lock.get_mut(&ip) {
            node.update_info(machine_info);
        }
    }

    /// Remove a node from the data store
    pub(crate) fn remove_node(&mut self, ip: &Ipv4Addr) {
        let mut node_lock = self.nodes.write().unwrap();
        node_lock.remove(ip);
    }
}
