//! the bot half of tet, that interfaces with users

mod handler;

use super::Tet;
use failure::Error;

pub(crate) use self::handler::Handler;

impl Tet {
    pub fn run_bot(&self) -> Result<(), Error> {
        let mut bot = match self.bot.lock() {
            Ok(bot) => bot,
            Err(not) => not.into_inner(),
        };
        if let Err(why) = bot.start() {
            eprintln!("Client error: {}", why);
        }
        Ok(())
    }
}
