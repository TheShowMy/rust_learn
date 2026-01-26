use std::time::SystemTime;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    pub room: String,
    pub username: String,
    pub timestamp: u64,
    pub message: MsgData,
}

impl Msg {
    pub fn new(room: String, username: String, message: MsgData) -> Self {
        Self {
            room,
            username,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            message,
        }
    }

    pub fn join(room: &str, username: &str) -> Self {
        Self::new(room.into(), username.into(), MsgData::Join)
    }

    pub fn leave(room: &str, username: &str) -> Self {
        Self::new(room.into(), username.into(), MsgData::Leave)
    }

    pub fn message(room: &str, username: &str, message: &str) -> Self {
        Self::new(
            room.into(),
            username.into(),
            MsgData::Message(message.into()),
        )
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MsgData {
    Join,
    Leave,
    Message(String),
}

//字符串切片转换为Msg结构体
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
