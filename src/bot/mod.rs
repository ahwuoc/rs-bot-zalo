pub mod bot;
pub mod error;
pub use error::ErrZalo;
pub const BASE_URL: &str = "https://bot-api.zaloplatforms.com/bot";

use serde::Deserialize;

#[derive(Debug, Clone)]
pub enum Method {
    Update,
    Send,
    SendPhoto,
    SendSticker,
    SendChatAction,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::Update => "getUpdates",
            Method::Send => "sendMessage",
            Method::SendPhoto => "sendPhoto",
            Method::SendSticker => "sendSticker",
            Method::SendChatAction => "sendChatAction",
        }
    }

    pub fn build_url(&self, token: &str) -> String {
        format!("{}{}/{}", BASE_URL, token, self.as_str())
    }
}

#[derive(Debug, Deserialize)]
pub struct Context {
    pub text: String,
    pub message_id: String,
    pub date: u64,
    pub from: FromTarget,
}

#[derive(Debug, Deserialize)]
pub struct FromTarget {
    pub id: String,
    pub display_name: String,
    pub is_bot: bool,
}

#[derive(Debug, Deserialize)]
pub struct MessageResponse {
    pub ok: bool,
    pub result: Option<ResultWrapper>,
    pub error_code: Option<i32>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResultWrapper {
    pub message: MessageInner,
    pub event_name: String,
}

#[derive(Debug, Deserialize)]
pub struct MessageInner {
    pub from: FromTarget,
    pub text: String,
    pub message_id: String,
    pub date: u64,
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    pub id: String,
    pub chat_type: String,
}
