use rs_bot_zalo::bot::bot::ZaloBot;

const TOKEN: &str = "YOUR_ZALO_BOT_TOKEN";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = ZaloBot::new(TOKEN);
    println!("Starting Echo Bot example...");
    bot.run(|ctx, bot| async move {
        println!("Nhận tin nhắn: '{}' từ {}", ctx.text, ctx.from.display_name);
        bot.send_message(&ctx.from.id, &format!("Bạn vừa nói: {}", ctx.text))
            .await?;
        if ctx.text.to_lowercase() == "sticker" {
            bot.send_sticker(&ctx.from.id, "12345").await?; // ID sticker mẫu
        }

        Ok(())
    })
    .await?;

    Ok(())
}
