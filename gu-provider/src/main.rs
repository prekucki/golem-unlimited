extern crate futures;
extern crate tokio;

extern crate actix;
extern crate actix_web;
extern crate clap;
extern crate gu_p2p;
extern crate gu_persist;

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate error_chain;
extern crate directories;

extern crate env_logger;
#[macro_use]
extern crate log;

extern crate rand;

mod hdman;
mod server;

const VERSION: &str = env!("CARGO_PKG_VERSION");

use clap::*;

fn main() {
    let matches = App::new("Golem Unlimited Provider")
        .version(VERSION)
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .arg(
            Arg::with_name("config-dir")
                .short("c")
                .takes_value(true)
                .value_name("PATH")
                .help("config dir path"),
        )
        .subcommand(server::clap_declare())
        .subcommand(SubCommand::with_name("status"))
        .get_matches();

    if ::std::env::var("RUST_LOG").is_err() {
        ::std::env::set_var("RUST_LOG", "info,gu_p2p=debug,gu_provider=debug")
    }
    env_logger::init();

    server::clap_match(&matches);
}