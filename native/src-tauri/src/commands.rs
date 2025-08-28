use crate::AppState;
use shared::store::data_store::{NodeData, NodeOverview};
use std::net::Ipv4Addr;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn get_nodes(app: AppHandle) -> Result<Vec<NodeOverview>, String> {
    let state = app.state::<AppState>();
    let read_lock = state.data_store.read().await;
    Ok(read_lock.get_node_overview())
}

#[tauri::command]
pub async fn get_node_detail(app: AppHandle, ip: Ipv4Addr) -> Result<Option<NodeData>, String> {
    let state = app.state::<AppState>();
    let read_lock = state.data_store.read().await;
    Ok(read_lock.get_node(ip))
}
