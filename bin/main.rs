extern crate config;
extern crate failure;
extern crate tet;

use failure::Error;
use tet::TetBot;

fn main() -> Result<(), Error> {
    // read config from various sources
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("tetcfg").required(false))?
        .merge(config::Environment::with_prefix("TET"))?;

    println!("settings: {:?}", settings);
    let bot_token = settings.get_str("bot_token")?;
    let mut bot = TetBot::init(bot_token)?;
    if let Err(why) = bot.client.start() {
    	println!("Client error: {}", why);
    }
    Ok(())
}
