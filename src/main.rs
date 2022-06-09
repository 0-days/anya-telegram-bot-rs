// use std::error::Error;
use teloxide::{prelude::*, utils::command::BotCommands, RequestError};
use teloxide::dptree::endpoint;

use dptree::{filter, Handler};

mod commands;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env().auto_send();

    // Bot::set_my_commands(bot, commands::Command);
    // teloxide::commands_repl(bot, commands::answer, commands::Command::ty()).await;

    // teloxide::repl(bot, |message: Message, bot: AutoSend<Bot>| async move {
    //     bot.send_dice(message.chat.id).await?;
    //     respond(())
    // })
    // .await;
    // let message_handler(
    //     bot: 
    // )
    // let cmd_handler = Update::filter_message().branch(endpoint(commands::answer));
    // let msg_handler = Update::filter_message().branch(endpoint(commands::msg_handler));
    let handler = Update::filter_message()
        // .branch(dptree::case![commands::Command::Ping]).endpoint(commands::ping)
        // .branch(dptree::case![commands::Command::Voice]).endpoint(commands::voice)
        // .branch(dptree::case![commands::Command::Echo(s)]).endpoint(commands::echo);
        .branch(dptree::entry().filter_command::<commands::Command>().endpoint(commands::answer))
        .branch(
            dptree::filter(|msg: Message| msg.text().is_some())
                .branch(filter(|| commands::roll_dice())).endpoint(commands::res_quote),
        );

    // let handler = dptree::entry()
        // .branch(filter_command().endpoint(commands::answer))
        // .branch(filter_command::<commands::Command, _>).endpoint(commands::answer);
        // .branch(filter_message()).endpoint(message_handler);
        // .branch(Update::filter_message()).endpoint(commands::answer);
        // .branch(cmd_handler)
        // .branch(msg_handler);

    Dispatcher::builder(bot, handler)
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}