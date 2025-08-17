use crate::commands::DiscoveryCommand;
use crate::store::data_store::DataStore;
use tracing::error;

pub struct ManagerServer {
    data_store: std::sync::Arc<tokio::sync::RwLock<DataStore>>,
    command_tx: tokio::sync::mpsc::Sender<DiscoveryCommand>,
    command_rx: tokio::sync::mpsc::Receiver<DiscoveryCommand>,
    response_tx: tokio::sync::broadcast::Sender<crate::schemas::target_messages::ResponseSchema>,
    #[allow(dead_code)]
    response_rx: tokio::sync::broadcast::Receiver<crate::schemas::target_messages::ResponseSchema>,
}

impl ManagerServer {
    pub fn new(date_store: std::sync::Arc<tokio::sync::RwLock<DataStore>>) -> Self {
        // channel for commands
        let (command_tx, command_rx): (
            tokio::sync::mpsc::Sender<DiscoveryCommand>,
            tokio::sync::mpsc::Receiver<DiscoveryCommand>,
        ) = tokio::sync::mpsc::channel(32);

        // channel for data
        let (response_tx, response_rx): (
            tokio::sync::broadcast::Sender<crate::schemas::target_messages::ResponseSchema>,
            tokio::sync::broadcast::Receiver<crate::schemas::target_messages::ResponseSchema>,
        ) = tokio::sync::broadcast::channel(32);

        ManagerServer {
            data_store: date_store,
            // The channel for commands
            command_tx,
            // The channel receive
            command_rx,
            // The channel for responses
            response_tx,
            // The channel to receive responses, this is a broadcast channel
            response_rx,
        }
    }

    pub async fn run(self) {
        // List to hold thread handlers
        let mut handlers = vec![];

        let data_store_command_tx = self.command_tx.clone();

        let discovery_server =
            crate::server::manager_threads::discovery_server::DiscoveryServer::new();
        let response_rx = self.response_tx.subscribe();
        let discover_server_handler = tokio::spawn(async move {
            discovery_server
                .run(self.command_rx, self.response_tx.clone())
                .await
        });
        handlers.push(discover_server_handler);

        // run data store service
        let data_store_for_service = self.data_store.clone();
        let data_store_service =
            crate::server::manager_threads::data_store_service::DataStoreService::new(
                data_store_for_service,
            );
        // let mut response_rx = self.response_tx.subscribe();
        data_store_service.run(data_store_command_tx, response_rx);

        for handler in handlers {
            if let Err(e) = handler.await {
                error!("Thread panicked: {:?}", e);
            }
        }
    }
}
