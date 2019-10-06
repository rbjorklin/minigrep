use std::env;
use std::process;
use structopt::StructOpt;

use minigrep::Config;
use minigrep::Opt;

fn main() {
    let opt = Opt::from_args();
    let config = Config::new(&opt, env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
