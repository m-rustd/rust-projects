use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Msg {
    pub room: String,
    pub username: String,
    pub timestamp: u64,
    pub data: MsgData,
}

// ======= msg data =======
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum MsgData {
    Join,
    Leave,
    Message(String),
}

impl TryFrom<&str> for Msg {
    type Error = serde_json::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value)
    }
}

impl TryFrom<&Msg> for String {
    type Error = serde_json::Error;

    fn try_from(value: &Msg) -> Result<Self, Self::Error> {
        serde_json::to_string(value)
    }
}

impl Msg {
    pub fn new(room: &str, username: &str, data: MsgData) -> Self {
        Msg {
            room: room.into(),
            username: username.into(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data,
        }
    }

    pub fn join(room: &str, username: &str) -> Self {
        Self::new(room, username, MsgData::Join)
    }

    pub fn leave(room: &str, username: &str) -> Self {
        Self::new(room, username, MsgData::Leave)
    }

    pub fn message(room: &str, username: &str, msg: &str) -> Self {
        Self::new(room, username, MsgData::Message(msg.into()))
    }
}