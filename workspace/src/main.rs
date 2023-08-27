extern crate basic;
extern crate env_logger;
extern crate log;

use basic::enums;

use env_logger::{Builder, Target};
use log::{debug, error, info};

fn init_log() {
    let mut builder = Builder::from_default_env();
    builder.target(Target::Stdout);
    builder.init();
}

fn main() {
    info!("Start to run with workspace");
    init_log();

    enums::run();

    info!("info log");
    debug!("debug log");
    error!("Failed to run");
}
