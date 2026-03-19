# rs-bot-zalo

[![Crates.io](https://img.shields.io/crates/v/rs-bot-zalo.svg)](https://crates.io/crates/rs-bot-zalo)
[![Documentation](https://docs.rs/rs-bot-zalo/badge.svg)](https://docs.rs/rs-bot-zalo)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple, powerful, and lightweight Rust library for building Zalo Bots using the [Zalo Bot API](https://developers.zalo.me/docs/api/official-account-api/tin-nhan/gui-tin-nhan-van-ban-post-4318).

This library leverages the **Arc Pattern** to efficiently share the bot instance across asynchronous handlers, ensuring high performance and minimal memory overhead.

## 🚀 Features

- **Asynchronous Architecture**: Built on top of `tokio` and `reqwest` for maximum efficiency.
- **Efficient Arc Sharing**: Pass the bot instance to handlers with near-zero cost.
- **Built-in Polling Loop**: Handles message polling and lifecycle management automatically.
- **Structured Error Handling**: Includes the `ErrZalo` enum for detailed API error management (401 Unauthorized, 429 Quota Exceeded, etc.).
- **Easy Integration**: Pre-wrapped methods for sending text, photos, and stickers.

## 📦 Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
rs-bot-zalo = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 🛠 Usage

Here is a simple **Echo Bot** implementation:

```rust
use rs_bot_zalo::bot::bot::ZaloBot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Initialize the bot with your Token
    let token = "YOUR_ZALO_BOT_TOKEN";
    let bot = ZaloBot::new(token);

    println!("Bot is running...");

    // 2. Start the bot with a handler
    // The handler receives (Context, ZaloBot)
    bot.run(|ctx, bot| async move {
        println!("Received: '{}' from {}", ctx.text, ctx.from.display_name);

        // Echo the received message back to the sender
        bot.send_message(&ctx.from.id, &format!("Echo: {}", ctx.text)).await?;

        Ok(())
    })
    .await?;

    Ok(())
}
```

## 📚 Supported Methods

- `send_message(chat_id, text)`: Send text messages.
- `send_photo(chat_id, url, caption)`: Send photos with captions.
- `send_sticker(chat_id, sticker_id)`: Send stickers.

## ⚠️ Error Handling

The library automatically returns an `ErrZalo` if the API request fails (e.g., token expired or invalid).

```rust
if let Err(e) = bot.run(handler).await {
    if let Some(zalo_err) = e.downcast_ref::<rs_bot_zalo::bot::ErrZalo>() {
         eprintln!("Zalo API Error: {}", zalo_err);
    }
}
```

## 📄 License

This library is licensed under the [MIT License](LICENSE).
