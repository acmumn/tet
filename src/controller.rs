use config::Config;
use diesel::prelude::*;
use failure::{err_msg, Error};
use serenity::prelude::*;
use std::sync::Mutex;

use bot::Handler;

pub struct Tet {
    pub(crate)config: Config,
    pub(crate) db: Mutex<SqliteConnection>,
    pub(crate) bot: Mutex<Client>,
}

impl Tet {
    pub fn new(config: Config) -> Result<Self, Error> {
        let db_url = config.get_str("database_url")?;
        let db = Mutex::new(SqliteConnection::establish(&db_url)?);

        let bot_token = config.get_str("bot_token")?;
        let bot;
        match Client::new(&bot_token, Handler) {
            Ok(value) => bot = Mutex::new(value),
            Err(error) => return Err(err_msg(format!("Failed to connect to Discord: {}", error))),
        };

        Ok(Tet { config, db, bot })
    }
}
