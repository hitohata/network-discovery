use crate::store::data_store::DataStore;
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use tracing::error;

pub struct DataStoreService {
    data_store: std::sync::Arc<Mutex<DataStore>>,
}

impl DataStoreService {
    pub fn new(data_store: Arc<Mutex<DataStore>>) -> Self {
        Self { data_store }
    }

    /// Run manager data store service.
    /// This service assume run eternal in a separate thread.
    pub fn run(&self, response_tx: Receiver<shared::schemas::target_messages::ResponseSchema>) {
        let data_store = self.data_store.clone();
        std::thread::spawn(move || {
            DataStoreService::watch_response(data_store.clone(), response_tx)
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
}
