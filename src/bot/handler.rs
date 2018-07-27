use serenity::model::gateway::Ready;
use serenity::prelude::*;

pub(crate) struct Handler;

impl EventHandler for Handler {
    fn ready(&self, _: Context, ready: Ready) {
        println!("connected to {}", ready.user.name);
    }
}
