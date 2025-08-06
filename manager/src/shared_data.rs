#[allow(clippy::all)]
#[allow(dead_code)]
pub struct DataEvent {
    message: String,
    timestamp: u64,
}

impl DataEvent {
    #[allow(dead_code)]
    #[allow(clippy::all)]
    pub fn new(message: String) -> Self {
        let timestamp = chrono::Utc::now().timestamp_millis() as u64;
        Self { message, timestamp }
    }
}

#[allow(dead_code)]
#[allow(clippy::all)]
pub struct SharedData {
    events: Vec<String>,
}
