pub struct DataEvent {
    message: String,
    timestamp: u64,
}

impl DataEvent {
    pub fn new(message: String) -> Self {
        let timestamp = chrono::Utc::now().timestamp_millis() as u64;
        Self { message, timestamp }
    }
}

pub struct SharedData {
    events: Vec<String>,
}
