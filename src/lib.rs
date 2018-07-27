extern crate failure;
#[macro_use]
extern crate serenity;

#[doc(inline)]
mod backend;
#[doc(inline)]
mod bot;

pub use bot::TetBot;
