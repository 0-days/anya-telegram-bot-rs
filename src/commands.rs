use std::error::Error;

use teloxide::{ prelude::*, utils::command::BotCommands, types::InputFile, types::ParseMode };
use teloxide::{ utils::markdown::{ link, bold } };
use rand::{ Rng, thread_rng };

mod voices;
mod auth;

#[derive(BotCommands, Clone)]
#[command(
    rename = "lowercase",
    description = "Commands",
    parse_with = "split"
)]
pub enum Command {
    #[command(description = "PING")]
    Ping,
    #[command(description = "The kawaii voice messages")]
    Voice,
    #[command(description = "Display invite link")]
    Invite,
    #[command(description = "Display group bio")]
    Bio,
    #[command()]
    Echo(String),
    #[command()]
    Bold(String),
    #[command()]
    Link(String, String)
    // #[command(description = "Exe")]
}

pub async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    match command {
        Command::Ping => {
            match message.from() {
                Some(user) => {
                    match &user.username {
                        Some(uname) if auth::is_papa(uname) => bot.send_message(message.chat.id, "pong").await?,
                        Some(_) => bot.send_message(message.chat.id, format!("アーニャおまえのことしらない")).await?,
                        None => bot.send_message(message.chat.id, "アーニャ… わからない").await?,
                    }
                }
                None => bot.send_message(message.chat.id, "アーニャ… わからない").await?,
            }
        }
        Command::Voice => {
            const VOICES_PATH: &str = "voices";
            bot.send_voice(message.chat.id, InputFile::file(format!("{}/{}", VOICES_PATH, voices::choose_voice()))).await?
        }
        Command::Invite => {
            match message.sender_chat() {
                Some(chat) => {
                    match chat.invite_link() {
                        Some(link) => bot.send_message(message.chat.id, format!("りんくそんざいした！\n{}", link)).await?,
                        None => bot.send_message(message.chat.id, "りんくそんざいしない").await?,
                    }
                }
                None => bot.send_message(message.chat.id, "りんくそんざいしない").await?,
            }
        }
        Command::Bio => {
            match message.chat.bio() {
                Some(bio) => bot.send_message(message.chat.id, format!("せつめいそんざいした！\nせつめい： {}", bio)).await?,
                None => bot.send_message(message.chat.id, "せつめいそんざいしない").await?,
            }
        }
        Command::Echo(text) => {
            // let command = Command::parse(message.text().unwrap(), "").unwrap();
            bot.send_message(message.chat.id, text).await?
        }
        Command::Bold(text) => {
            bot.send_message(message.chat.id, bold(&text))
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
        Command::Link(caption, url) => {
            bot.send_message(message.chat.id, format!("▫️{}\n", link(&url, &caption)))
                .reply_to_message_id(message.id)
                .parse_mode(ParseMode::MarkdownV2)
                .await?
        }
    };

    Ok(())
}

fn quote(m: Message) -> String {
    const Q01: &str = "ものすごいうそつき　でも … かっこいいうそつき！";
    const Q02: &str = "だいじょうぶます\nこわくない";
    const Q03F: &str = "アーニャおうちかえりたい";
    const Q03L: &str = "とアーニャのおうち";
    const Q04: &str = "アーニャじゃまなこども？";

    let mut rng = thread_rng();
    let roll: u8 = rng.gen_range(0..4);
    return match roll {
        0 => {
            let name = m.from().unwrap().first_name.as_str();
            format!("{name}{Q01}")
        },
        1 => Q02.to_string(),
        2 => {
            let name = m.from().unwrap().first_name.as_str();
            format!("{Q03F}{name}{Q03L}")
        },
        3 => Q04.to_string(),
        _ => unreachable!(),
    }
}
pub async fn res_quote(b: AutoSend<Bot>, m: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    // let mut rng = thread_rng();
    // let roll: bool = rng.gen_bool(1.0 / 10.0);
    // if roll {
    b.send_message(m.chat.id, quote(m)).await?;
    // }
    Ok(())
}

// pub fn roll_dice() -> bool {
//     let mut rng = thread_rng();
//     rng.gen_bool(1.0 / 10.0)
// }