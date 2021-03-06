#![recursion_limit = "1024"]

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate log;
extern crate serenity;

extern crate serde_json;
extern crate serde;

mod error;
mod config;
mod logging;
mod events;
mod commands;

use error::Result;
use config::Config;
use logging::Logger;

use serenity::client;
use serenity::client::{Client, ClientError};
use serenity::Error;

fn main() {
    std::process::exit(match actual_main() {
        Ok(_) => 0,
        Err(err) => {
            error!("Error in main: {}", err);
            1
        }
    });
}

fn actual_main() -> Result<()> {
    // todo: add commands
    Logger::init()?;
    let config = Config::from_file("config.json")?;
    match client::validate_token(&config.token) {
        Ok(()) => {},
        Err(Error::Client(ClientError::InvalidToken)) => {
            error!("Error when validating token, please ensure you are using a valid bot token.");
            std::process::exit(1)
        },
        Err(why) => {
            error!("Unexpected error: {:?}", why);
            std::process::exit(1)
        }
    }
    let mut client = Client::login_bot(&config.token);
    client.on_guild_member_add(events::on_member_join);
    client.start()?;
    Ok(())
}
