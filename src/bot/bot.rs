use std::sync::Arc;

use reqwest::Client;
use serde_json::json;
use tokio::time::{Duration, sleep};

use crate::bot::{Context, MessageResponse, Method};

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
    ) -> Result<MessageResponse, Box<dyn std::error::Error>> {
        let url = method.build_url(&self.inner.token);
        let res = self.inner.client.post(url).json(&body).send().await?;

        let data: MessageResponse = res.json().await?;
        if !data.ok {
            let code = data.error_code.unwrap_or(0);
            return Err(Box::new(crate::bot::ErrZalo::from_code(code)));
        }
        Ok(data)
    }

    async fn poll_once(&self) -> Result<Option<Context>, Box<dyn std::error::Error>> {
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

    pub async fn run<F, Fut>(&self, handler: F) -> Result<(), Box<dyn std::error::Error>>
    where
        F: Fn(Context, ZaloBot) -> Fut + Send + Sync,
        Fut: std::future::Future<Output = Result<(), Box<dyn std::error::Error>>> + Send,
    {
        println!("Bot started...");
        loop {
            match self.poll_once().await {
                Ok(Some(ctx)) => {
                    if let Err(e) = handler(ctx, self.clone()).await {
                        eprintln!("[Handler Error]: {}", e);
                    }
                }
                Ok(None) => {}
                Err(e) => {
                    eprintln!("[Poll Error]: {}", e);
                    if let Some(zalo_err) = e.downcast_ref::<crate::bot::ErrZalo>() {
                        if matches!(zalo_err, crate::bot::ErrZalo::Unauthorized) {
                            println!("Unauthorized (401): Stopping bot. Please check your token.");
                            return Err(e);
                        }
                    }
                }
            }
            sleep(Duration::from_secs(1)).await;
        }
    }

    pub async fn send_message(
        &self,
        chat_id: &str,
        text: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

    pub async fn send_photo(
        &self,
        chat_id: &str,
        photo: &str,
        caption: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

    pub async fn send_sticker(
        &self,
        chat_id: &str,
        sticker_id: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
