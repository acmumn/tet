use config::Config;
use diesel::prelude::*;
use failure::{err_msg, Error};
use serenity::prelude::*;
use std::sync::Mutex;

use bot::Handler;

pub struct Tet {
    db: Mutex<SqliteConnection>,
    bot: Mutex<Client>,
}

impl Tet {
    pub fn new(settings: Config) -> Result<Self, Error> {
        let db_url = settings.get_str("database_url")?;
        let db = Mutex::new(SqliteConnection::establish(&db_url)?);

        println!("settings: {:?}", settings);
        let bot_token = settings.get_str("bot_token")?;
        let bot;
        match Client::new(&bot_token, Handler) {
            Ok(value) => bot = Mutex::new(value),
            Err(error) => return Err(err_msg(format!("Failed to connect to Discord: {}", error))),
        };

        Ok(Tet { db, bot })
    }
    pub fn run_scheduler(&self) -> Result<(), Error> {
        Ok(())
    }
    pub fn run_bot(&self) -> Result<(), Error> {
        Ok(())
    }
}
