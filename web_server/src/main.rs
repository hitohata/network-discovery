use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{routing, Json};
use shared::store::data_store::{DataStore, DataStoreType};
use std::sync::Arc;
use tracing::Level;

struct AppState {
    // Add shared state here if needed
    data_store: DataStoreType,
}

#[tokio::main]
async fn main() {
    // set up tracing for logging
    tracing_subscriber::fmt()
        .with_env_filter(Level::INFO.as_str())
        .with_thread_ids(true)
        .with_thread_names(true)
        .init();

    let data_store = DataStore::init();

    // run manager server
    let data_store_for_server = data_store.clone();
    let manager_server = shared::server::manager_server::ManagerServer::new(data_store_for_server);
    let manager_server_handler = tokio::spawn(async move {
        manager_server.run().await;
    });

    let shared_state = Arc::new(AppState {
        data_store: Arc::clone(&data_store),
    });

    let app = axum::Router::new()
        .route("/", routing::get(|| async { "Hello, World!" }))
        .route("/nodes", routing::get(node_overview))
        .route("/nodes/{ip}", routing::get(get_node))
        .with_state(shared_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn node_overview(
    State(state): State<Arc<AppState>>,
) -> Json<Vec<crate::return_type::NodesData>> {
    let store = state.data_store.read().await;
    let nodes = store.get_node_overview();
    drop(store);
    Json(
        nodes
            .iter()
            .map(|node| {
                crate::return_type::NodesData::new(
                    node.ip,
                    node.machine_info.clone(),
                    node.usage.clone(),
                    node.last_updated
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                )
            })
            .collect::<Vec<crate::return_type::NodesData>>(),
    )
}

async fn get_node(
    Path(ip): Path<String>,
    State(state): State<Arc<AppState>>,
) -> Result<Json<Option<crate::return_type::Node>>, StatusCode> {
    let store = state.data_store.read().await;
    let ip = match ip.parse::<std::net::Ipv4Addr>() {
        Ok(ip) => ip,
        Err(_) => {
            return Err(StatusCode::BAD_REQUEST);
        }
    };
    let node = store.get_node(ip);
    drop(store);
    Ok(Json(node.map(|node| {
        return_type::Node::new(
            node.ip,
            node.machine_info,
            node.usage,
            node.last_updated
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        )
    })))
}

mod return_type {
    use shared::store::data_store::MachineUsageData;

    #[derive(Debug, Clone, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct NodesData {
        ip: std::net::Ipv4Addr,
        machine_info: Option<shared::schemas::device_info::MachineInfo>,
        usage: Option<shared::schemas::device_info::MachineUsage>,
        last_updated: u64,
    }
    impl NodesData {
        pub fn new(
            ip: std::net::Ipv4Addr,
            machine_info: Option<shared::schemas::device_info::MachineInfo>,
            usage: Option<shared::schemas::device_info::MachineUsage>,
            last_updated: u64,
        ) -> Self {
            Self {
                ip,
                machine_info,
                usage,
                last_updated,
            }
        }
    }

    #[derive(Debug, Clone, serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Node {
        ip: std::net::Ipv4Addr,
        machine_info: Option<shared::schemas::device_info::MachineInfo>,
        usage: Vec<MachineUsageData>,
        last_updated: u64,
    }
    impl Node {
        pub fn new(
            ip: std::net::Ipv4Addr,
            machine_info: Option<shared::schemas::device_info::MachineInfo>,
            usage: Vec<MachineUsageData>,
            last_updated: u64,
        ) -> Self {
            Self {
                ip,
                machine_info,
                usage,
                last_updated,
            }
        }
    }
}
