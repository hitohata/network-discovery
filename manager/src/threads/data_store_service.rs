use crate::store::data_store::DataStore;
use std::ops::Sub;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use tracing::{error, info};

pub struct DataStoreService {
    data_store: std::sync::Arc<Mutex<DataStore>>,
}

const THRESHOLD: u64 = 30;
const CHECK_FREQUENCY: u64 = 10;

impl DataStoreService {
    pub fn new(data_store: Arc<Mutex<DataStore>>) -> Self {
        Self { data_store }
    }

    /// Run manager data store service.
    /// This service assume run eternal in a separate thread.
    pub fn run(&self, response_tx: Receiver<shared::schemas::target_messages::ResponseSchema>) {
        let ds_4_receive = self.data_store.clone();
        std::thread::spawn(move || {
            DataStoreService::watch_response(ds_4_receive.clone(), response_tx)
        });
        let ds_4_check = self.data_store.clone();
        std::thread::spawn(move || {
            DataStoreService::check_lost_connection(ds_4_check.clone());
        });
    }

    /// check a response and update the data store accordingly.
    fn watch_response(
        data_store: std::sync::Arc<Mutex<DataStore>>,
        response_tx: Receiver<shared::schemas::target_messages::ResponseSchema>,
    ) {
        loop {
            match response_tx.recv() {
                Ok(res) => match res {
                    shared::schemas::target_messages::ResponseSchema::Spec(spec_response) => {
                        let mut data_store = data_store.lock().unwrap();
                        // TODO: to event
                        info!(
                            "New node find: {:?} / {:?}",
                            spec_response.ip, spec_response.spec.host_name
                        );
                        data_store.update_node_information(spec_response.ip, spec_response.spec);
                    }
                    shared::schemas::target_messages::ResponseSchema::UsageOverview(
                        usage_response,
                    ) => {
                        let mut lock = data_store.lock().unwrap();
                        lock.update_usage(usage_response.ip, usage_response.usage);
                    }
                },
                Err(e) => {
                    error!("Failed to receive response: {}", e);
                    continue;
                }
            }
        }
    }

    /// check removed nodes
    fn check_lost_connection(data_store: std::sync::Arc<Mutex<DataStore>>) {
        loop {
            // read node
            let lock = data_store.lock().unwrap();
            let nodes = lock.get_nodes();
            drop(lock);

            let now = SystemTime::now();
            let threshold = now.sub(Duration::from_secs(THRESHOLD));

            for node in nodes.iter() {
                if node.last_updated < threshold {
                    let mut lock = data_store.lock().unwrap();
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

            std::thread::sleep(std::time::Duration::from_secs(CHECK_FREQUENCY));
        }
    }
}
