// use std::error::Error;
use teloxide::{ prelude::* };
use teloxide::dptree::endpoint;

mod commands;
mod youtube;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting throw dice bot...");

    let bot = Bot::from_env().auto_send();

    // let cmd_handler = Update::filter_message().branch(endpoint(commands::answer));
    // let msg_handler = Update::filter_message().branch(endpoint(commands::msg_handler));
    let handler = Update::filter_message()
        .branch(dptree::entry().filter_command::<commands::Command>().endpoint(commands::answer))
        .branch(
            dptree::filter(|msg: Message| msg.text().is_some())
                // .branch(filter(|| commands::roll_dice())).endpoint(commands::res_quote),
                    .branch(dptree::filter(|m: Message| youtube::contains_yt(m))).endpoint(youtube::answer)
                    .branch(endpoint(commands::res_quote)),
        );

    Dispatcher::builder(bot, handler)
        .build()
        .setup_ctrlc_handler()
        .dispatch()
        .await;
}