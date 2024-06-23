use teloxide::prelude::*;
use teloxide::types::Message;

async fn handle_new_members(bot: Bot, msg: Message) -> ResponseResult<()> {
    if let Some(new_members) = msg.new_chat_members() {
        for new_member in new_members {
            let username = new_member.username.as_ref()
                .map(|name| format!("@{name}"))
                .unwrap_or_else(|| {
                    format!("{} {}",
                            new_member.first_name,
                            new_member.last_name.as_deref().unwrap_or(""))
                });

            let group_name = msg.chat.title().unwrap_or("Unknown group");

            let welcome_message = format!(
                "Ласкаво просимо до групи, {username}! 🌟\n\n\
                Ми раді вітати вас серед нас. Ось кілька порад для початку:\n\n\
                1. Представтеся: Розкажіть нам трохи про себе! Що привело вас сюди?\n\
                2. Правила Групи: Обов'язково ознайомтеся з нашими правилами групи, щоб зберегти нашу спільноту дружньою та шанобливою.\n\
                3. Беріть участь та взаємодійте: Не соромтеся задавати питання, ділитися своїми думками та брати участь в обговореннях.\n\n\
                Якщо у вас виникнуть питання, не соромтеся звертатися. Ми всі тут, щоб допомагати один одному рости та досягати успіху!\n\n\
                Насолоджуйтесь перебуванням тут!\n\n\
                З найкращими побажаннями,\n\
                {group_name}"
            );

            bot.send_message(msg.chat.id, welcome_message).await?;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting greeter bot...");

    let bot = Bot::from_env();

    let handler = Update::filter_message().branch(
        dptree::filter(|msg: Message| msg.new_chat_members().is_some())
            .endpoint(handle_new_members)
    );

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}