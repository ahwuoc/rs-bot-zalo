use rs_bot_zalo::{ZaloBot, ZaloResult};

const TOKEN: &str = "YOUR_ZALO_BOT_TOKEN";

#[tokio::main]
async fn main() -> ZaloResult<()> {
    // Khởi tạo logging để xem các thông báo từ thư viện
    tracing_subscriber::fmt::init();

    let bot = ZaloBot::new(TOKEN);
    println!("Đang chạy Echo Bot...");

    bot.run(|ctx, bot| async move {
        println!("Nhận tin nhắn: '{}' từ {}", ctx.text, ctx.from.display_name);

        // Phản hồi lại tin nhắn
        bot.send_message(&ctx.from.id, &format!("Bạn vừa nói: {}", ctx.text))
            .await?;

        // Tặng thêm cái sticker nếu thích
        if ctx.text.to_lowercase().contains("vui") {
            bot.send_sticker(&ctx.from.id, "12345").await?;
        }

        Ok(())
    })
    .await?;

    Ok(())
}
