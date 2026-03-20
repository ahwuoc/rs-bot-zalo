use reqwest::Client;
use serde_json::json;
use std::sync::Arc;
use tokio::time::{Duration, sleep};

use crate::bot::{Context, ErrZalo, MessageResponse, Method, ZaloError};

pub type ZaloResult<T> = Result<T, ZaloError>;

#[derive(Debug)]
struct BotInner {
    client: Client,
    token: String,
}

#[derive(Clone, Debug)]
pub struct ZaloBot {
    inner: Arc<BotInner>,
}

impl ZaloBot {
    pub fn new(token: &str) -> Self {
        Self {
            inner: Arc::new(BotInner {
                client: Client::new(),
                token: token.to_string(),
            }),
        }
    }

    async fn post_request<S: serde::Serialize>(
        &self,
        method: Method,
        body: S,
    ) -> ZaloResult<MessageResponse> {
        let url = method.build_url(&self.inner.token);
        let res = self.inner.client.post(url).json(&body).send().await?;

        let data: MessageResponse = res.json().await?;
        if !data.ok {
            let code = data.error_code.unwrap_or(0);
            return Err(ZaloError::Api(ErrZalo::from_code(code)));
        }
        Ok(data)
    }
    async fn poll_once(&self) -> ZaloResult<Option<Context>> {
        let data = self
            .post_request(Method::Update, json!({ "timeout": 30 }))
            .await?;

        if let Some(result) = data.result {
            let ctx = Context {
                text: result.message.text,
                message_id: result.message.message_id,
                date: result.message.date,
                from: result.message.from,
            };
            return Ok(Some(ctx));
        }
        Ok(None)
    }

    pub async fn run<F, Fut>(&self, handler: F) -> ZaloResult<()>
    where
        F: Fn(Context, ZaloBot) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = ZaloResult<()>> + Send,
    {
        tracing::info!("Starting Zalo bot loop...");
        loop {
            match self.poll_once().await {
                Ok(Some(ctx)) => {
                    if let Err(e) = handler(ctx, self.clone()).await {
                        tracing::error!("Handler error: {}", e);
                    }
                }
                Ok(None) => {}
                Err(ZaloError::Api(ErrZalo::Unauthorized)) => {
                    tracing::error!("Unauthorized (401): Stopping bot. Please check your token.");
                    return Err(ZaloError::Api(ErrZalo::Unauthorized));
                }
                Err(e) => {
                    tracing::warn!("Polling error: {}. Retrying in 5 seconds...", e);
                    sleep(Duration::from_secs(5)).await;
                    continue;
                }
            }
            sleep(Duration::from_millis(500)).await;
        }
    }

    pub async fn send_message(&self, chat_id: &str, text: &str) -> ZaloResult<()> {
        self.post_request(
            Method::Send,
            json!({
                "chat_id": chat_id,
                "text": text
            }),
        )
        .await?;
        Ok(())
    }

    pub async fn send_photo(&self, chat_id: &str, photo: &str, caption: &str) -> ZaloResult<()> {
        self.post_request(
            Method::SendPhoto,
            json!({
                "chat_id": chat_id,
                "photo": photo,
                "caption": caption,
            }),
        )
        .await?;
        Ok(())
    }
    pub async fn send_sticker(&self, chat_id: &str, sticker_id: &str) -> ZaloResult<()> {
        self.post_request(
            Method::SendSticker,
            json!({
                "chat_id": chat_id,
                "sticker": sticker_id,
            }),
        )
        .await?;
        Ok(())
    }
}
