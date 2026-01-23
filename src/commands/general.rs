
/*========== Used {Serenity} ===========*/

use serenity::all::MessageBuilder;
use serenity::framework::standard::macros::command;
use serenity::framework::standard::CommandResult;
use serenity::model::prelude::*;
use serenity::prelude::*;

/*======================================*/

/*======= Command {Ping} ========*/

#[command]
#[only_in(guilds)]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {

    let channel = msg.channel_id;

    let response = MessageBuilder::new()
        .push("User ")
        .push_bold_safe(&msg.author.name)
        .push(" used the 'ping' command in the ")
        .mention(&channel)
        .push(" channel")
        .build();

    msg.channel_id.say(&ctx.http, &response).await?;
    Ok(())
}

/*==============================*/

/*======= Command {Help} ========*/

#[command]
#[only_in(guilds)]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {

    let channel = msg.channel_id;

    use serenity::utils::MessageBuilder;

    let response = MessageBuilder::new()
        .push_bold_line("Доступные команды бота:")
        .push_line("") // Пустая строка для отступа

        .push_bold_line("🎶 Музыка:")
        .push_mono_line("!join <название> - Зайти в канал")
        .push_mono_line("!play <файл>    - Играть музыку из папки source")
        .push_mono_line("!stop           - Остановить всё и очистить очередь")
        .push_mono_line("!next           - Пропустить текущий трек")
        .push_mono_line("!leave          - Покинуть канал")
        .push_line("")

        .push_bold_line("🛡️ Администрирование:")
        .push_mono_line("!mute           - Выключить микрофон бота")
        .push_mono_line("!unmute         - Включить микрофон бота")
        .push_line("")

        .push_bold_line("⚙️ Общее:")
        .push_mono_line("!ping           - Проверить задержку")
        .push_mono_line("!help           - Показать это сообщение")
        .push_mono_line("!quit           - Выключить бота")
        .build();

    msg.channel_id.say(&ctx.http, &response).await?;
    Ok(())
}

/*==============================*/