#[macro_use]
extern crate diesel;
extern crate config;
extern crate chrono;
extern crate failure;
extern crate schedule;
extern crate serenity;

mod backend;
mod bot;
mod controller;

pub use controller::Tet;
