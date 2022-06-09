use std::env;

pub fn echo_token() {
  match env::var("TELEGRAM_BOT_TOKEN") {
    Ok(val) => println!("TOKEN: {val:?}"),
    Err(e) => println!("couldn't interpret TOKEN: {e}"),
  }
}