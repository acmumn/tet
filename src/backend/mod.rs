mod models;
mod schema;

use std::thread;
use std::time::Duration;

use schedule::Agenda;
use failure::{err_msg, Error};

use super::Tet;

impl Tet {
    pub fn run_scheduler(&self) -> Result<(), Error> {
        // determine interval
    	let interval_signed = self.config.get_int("interval").unwrap_or(5000);
    	if interval_signed < 0 {
    		return Err(err_msg(format!("Interval should not be negative (interval = {}).", interval_signed)));
    	}
    	let interval = interval_signed as u64;
        let mut agenda = Agenda::new();

        loop {
            agenda.run_pending();
        	thread::sleep(Duration::from_millis(interval));
        }
    }
}
