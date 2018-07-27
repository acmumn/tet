extern crate config;
extern crate failure;
extern crate tet;

use std::sync::Arc;
use std::thread;

use failure::Error;
use tet::Tet;

fn main() -> Result<(), Error> {
    // read config from various sources
    let mut settings = config::Config::default();
    settings
        .merge(config::File::with_name("tetcfg").required(false))?
        .merge(config::Environment::with_prefix("TET"))?;

    let tet = Arc::new(Tet::new(settings)?);

    let tet_sched = tet.clone();
    thread::spawn(move || tet_sched.run_scheduler());

    tet.run_bot()
}
