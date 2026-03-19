# rs-bot-zalo

A simple and powerful Rust library for building Zalo Bots using the [Zalo Bot API](https://developers.zalo.me/docs/api/official-account-api/tin-nhan/gui-tin-nhan-van-ban-post-4318).

This library uses the **Arc Pattern** to share the bot instance efficiently across asynchronous handlers, making it lightweight and high-performance.

## 🚀 Tính năng nổi bật (Features)

- **Asynchronous Design**: Được xây dựng trên `tokio` và `reqwest`.
- **Arc sharing**: Share bot cho handler cực kỳ rẻ, không lo về bộ nhớ.
- **Built-in Loop**: Tự động polling tin nhắn và quản lý vòng lặp bot.
- **Error Handling**: Xử lý lỗi từ Zalo API bài bản (với Enum `ErrZalo`).
- **Easy to use**: Đã đóng gói sẵn các method gửi tin nhắn, gửi ảnh, gửi sticker.

## 📦 Cài đặt (Installation)

Thêm vào `Cargo.toml` của bạn:

```toml
[dependencies]
rs-bot-zalo = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

## 🛠 Cách sử dụng (Usage)

Dưới đây là ví dụ về một **Echo Bot** đơn giản:

```rust
use rs_bot_zalo::bot::bot::ZaloBot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let token = "YOUR_ZALO_BOT_TOKEN";
    let bot = ZaloBot::new(token);

    // Chạy bot và đăng ký handler
    bot.run(|ctx, bot| async move {
        println!("Tin nhắn: {} từ {}", ctx.text, ctx.from.display_name);

        // Phản hồi lại tin nhắn
        bot.send_message(&ctx.from.id, &format!("Echo: {}", ctx.text)).await?;

        Ok(())
    })
    .await?;

    Ok(())
}
```

## 📚 Các Method được hỗ trợ

- `send_message(chat_id, text)`: Gửi tin nhắn văn bản.
- `send_photo(chat_id, url, caption)`: Gửi ảnh kèm chú thích.
- `send_sticker(chat_id, sticker_id)`: Gửi sticker.

## ⚠️ Xử lý lỗi

Thư viện tự động trả về lỗi `ErrZalo` nếu API thất bại (ví dụ: Token hết hạn).

```rust
match bot.run(handler).await {
    Err(e) => {
        if let Some(zalo_err) = e.downcast_ref::<rs_bot_zalo::bot::ErrZalo>() {
             println!("Lỗi Zalo: {}", zalo_err);
        }
    },
    _ => ()
}
```

## 📄 License

Thư viện này được phát hành dưới bản quyền [MIT](LICENSE).
