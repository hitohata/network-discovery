use crate::store::data_store::DataStoreType;
use std::ops::Sub;
use std::time::{Duration, SystemTime};
use tracing::{error, info};

pub struct DataStoreService {
    data_store: DataStoreType,
}

const THRESHOLD: u64 = 30;
const CHECK_FREQUENCY: u64 = 10;

impl DataStoreService {
    pub fn new(data_store: DataStoreType) -> Self {
        Self { data_store }
    }

    /// Run manager data store service.
    /// This service assume run eternal in a separate thread.
    pub fn run(
        &self,
        command_tx: tokio::sync::mpsc::Sender<crate::commands::DiscoveryCommand>,
        response_tx: tokio::sync::broadcast::Receiver<
            crate::schemas::target_messages::ResponseSchema,
        >,
    ) {
        let data_store_receive = self.data_store.clone();
        tokio::spawn(async move {
            DataStoreService::watch_response(command_tx, data_store_receive, response_tx).await;
        });

        let ds_4_check = self.data_store.clone();
        std::thread::spawn(async move || {
            DataStoreService::check_lost_connection(ds_4_check.clone()).await
        });
    }

    /// check a response and update the data store accordingly.
    async fn watch_response(
        command_tx: tokio::sync::mpsc::Sender<crate::commands::DiscoveryCommand>,
        data_store: DataStoreType,
        mut response_tx: tokio::sync::broadcast::Receiver<
            crate::schemas::target_messages::ResponseSchema,
        >,
    ) {
        loop {
            match response_tx.recv().await {
                Ok(res) => {
                    match res {
                        crate::schemas::target_messages::ResponseSchema::Spec(spec_response) => {
                            let mut data_store = data_store.write().await;
                            // TODO: to event
                            info!(
                                "New node find: {:?} / {:?}",
                                spec_response.ip, spec_response.spec.host_name
                            );
                            data_store
                                .update_node_information(spec_response.ip, spec_response.spec);
                        }
                        crate::schemas::target_messages::ResponseSchema::UsageOverview(
                            usage_response,
                        ) => {
                            let mut lock = data_store.write().await;

                            // get current node
                            let node = lock.get_node(usage_response.ip);

                            // write data
                            lock.update_usage(usage_response.ip, usage_response.usage);
                            drop(lock);

                            // if the current node is None, it means this is a new node
                            if node.is_none() {
                                if let Err(e) = command_tx
                                    .send(crate::commands::DiscoveryCommand::DeviceInformation(
                                        usage_response.ip,
                                    ))
                                    .await
                                {
                                    error!("Failed to send Spec request: {}", e);
                                };
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to receive response: {}", e);
                    continue;
                }
            }
        }
    }

    /// check removed nodes
    async fn check_lost_connection(data_store: DataStoreType) {
        loop {
            // read node
            let lock = data_store.read().await;
            let nodes = lock.get_nodes();
            drop(lock);

            let now = SystemTime::now();
            let threshold = now.sub(Duration::from_secs(THRESHOLD));

            for node in nodes.iter() {
                if node.last_updated < threshold {
                    let mut lock = data_store.write().await;
                    lock.remove_node(&node.ip);
                    drop(lock);

                    let ip = &node.ip;
                    // TODO to
                    let node_name = match &node.machine_info {
                        Some(info) => info.host_name.as_str(),
                        None => "No Name",
                    };
                    info!("The node removed: {:?} / {:?}", ip, node_name);
                }
            }

            tokio::time::sleep(std::time::Duration::from_secs(CHECK_FREQUENCY)).await;
        }
    }
}
