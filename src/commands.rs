use std::error::Error;

use teloxide::{prelude::*, utils::command::BotCommands, types::InputFile, RequestError};
use rand::{Rng, thread_rng};

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
    };

    Ok(())
}

// pub async fn answer(
//     bot: AutoSend<Bot>,
//     msg: Message,
// ) -> Result<(), RequestError> {
//     if msg.text().is_none() {
//         return respond(());
//     }

//     match Command::parse(msg.text().unwrap(), "") {
//         // match command {
//         Ok(Command::Ping) => {
//             match msg.from() {
//                 Some(user) => {
//                     match &user.username {
//                         Some(uname) if auth::is_papa(uname) => bot.send_message(msg.chat.id, "pong").await?,
//                         Some(_) => bot.send_message(msg.chat.id, format!("アーニャおまえのことしらない")).await?,
//                         None => bot.send_message(msg.chat.id, "アーニャ… わからない").await?,
//                     }
//                 }
//                 None => bot.send_message(msg.chat.id, "アーニャ… わからない").await?,
//             }
//         }
//         Ok(Command::Voice) => {
//             const VOICES_PATH: &str = "voices";
//             bot.send_voice(msg.chat.id, InputFile::file(format!("{}/{}", VOICES_PATH, voices::choose_voice()))).await?
//         }
//         Ok(Command::Invite) => {
//             match msg.sender_chat() {
//                 Some(chat) => {
//                     match chat.invite_link() {
//                         Some(link) => bot.send_message(msg.chat.id, format!("りんくそんざいした！\n{}", link)).await?,
//                         None => bot.send_message(msg.chat.id, "りんくそんざいしない").await?,
//                     }
//                 }
//                 None => bot.send_message(msg.chat.id, "りんくそんざいしない").await?,
//             }
//         }
//         Ok(Command::Bio) => {
//             match msg.chat.bio() {
//                 Some(bio) => bot.send_message(msg.chat.id, format!("せつめいそんざいした！\nせつめい： {}", bio)).await?,
//                 None => bot.send_message(msg.chat.id, "せつめいそんざいしない").await?,
//             }
//         }
//         Ok(Command::Echo(text)) => {
//             // let command = Command::parse(message.text().unwrap(), "").unwrap();
//             bot.send_message(msg.chat.id, text).await?
//         }
        // _ => {
        //     let mut rng = thread_rng();
        //     let roll: bool = rng.gen_bool(1.0 / 10.0);
        //     if roll {
        //         bot.send_message(msg.chat.id, quote(msg)).await?;
        //     } else { return respond(()) }
        // }
        // };
        // }
        // Error => {
        //     let mut rng = thread_rng();
        //     let roll: bool = rng.gen_bool(1.0 / 10.0);
        //     if roll {
        //         bot.send_message(msg.chat.id, quote(msg)).await?;
        //     };
        //     // respond(());
        // }
    // };
    // respond(())

    // Ok(())
// }

pub async fn msg_handler(
    bot: AutoSend<Bot>,
    msg: Message,
) -> Result<(), RequestError> {
    if msg.text().is_none() {
        return respond(());
    }
    let mut rng = thread_rng();
    let roll: bool = rng.gen_bool(1.0 / 10.0);
    match roll {
        true => {
            bot.send_message(msg.chat.id, quote(msg)).await?;
        }
        false => {}
    };
    respond(())
}

fn quote(m: Message) -> String {
    const Q01: &str = "ものすごいうそつき　でも … かっこいいうそつき！";
    const Q02: &str = "だいじょうぶます\nこわくない";
    const Q03f: &str = "アーニャおうちかえりたい";
    const Q03l: &str = "とアーニャのおうち";
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
            format!("{Q03f}{name}{Q03l}")
        },
        3 => Q04.to_string(),
        _ => unreachable!(),
    }
}
pub async fn ping(
    b: AutoSend<Bot>,
    msg: Message,
    cmd: Command
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    match msg.from() {
        Some(user) => {
            match &user.username {
                Some(uname) if auth::is_papa(uname) => b.send_message(msg.chat.id, "pong").await?,
                Some(_) => b.send_message(msg.chat.id, format!("アーニャおまえのことしらない")).await?,
                None => b.send_message(msg.chat.id, "アーニャ… わからない").await?,
            }
        }
        None => b.send_message(msg.chat.id, "アーニャ… わからない").await?,
    };
    Ok(())
}
pub async fn voice(
    b: AutoSend<Bot>,
    msg: Message,
    cmd: Command
) -> Result<(), Box<dyn Error + Send + Sync>> {
    const VOICES_PATH: &str = "voices";
    b.send_voice(msg.chat.id, InputFile::file(format!("{}/{}", VOICES_PATH, voices::choose_voice()))).await?;
    Ok(())
}
pub async fn echo(
    b: AutoSend<Bot>,
    msg: Message,
    cmd: Command,
    text: String
) -> Result<(), Box<dyn Error + Send + Sync>> {
    b.send_message(msg.chat.id, text).await?;
    Ok(())
}
pub async fn res_quote(b: AutoSend<Bot>, m: Message) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut rng = thread_rng();
    let roll: bool = rng.gen_bool(1.0 / 10.0);
    if roll {
        b.send_message(m.chat.id, quote(m)).await?;
    }
    Ok(())
}

pub fn roll_dice() -> bool {
    let mut rng = thread_rng();
    rng.gen_bool(1.0 / 10.0)
}