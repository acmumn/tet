//! the bot half of tet, that interfaces with users

mod handler;

use failure::{err_msg, Error};
use serenity::prelude::*;

use self::handler::Handler;

pub struct TetBot {
    pub client: Client,
}

impl TetBot {
    pub fn init(token: String) -> Result<Self, Error> {
        let client;
        match Client::new(&token, Handler) {
            Ok(value) => client = value,
            Err(error) => return Err(err_msg(format!("Failed to connect to Discord: {}", error))),
        };
        Ok(TetBot { client })
    }
}
