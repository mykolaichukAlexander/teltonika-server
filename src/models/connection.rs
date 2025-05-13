
pub struct ConnectionState {
    pub imei: Option<String>,
    pub partial_ack: Option<u8>, // Store the first byte of a possibly split ACK
}

impl ConnectionState {
    pub fn new() -> Self {
        ConnectionState {
            imei: None,
            partial_ack: None,
        }
    }
}