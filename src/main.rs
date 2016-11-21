extern crate biddy;

use std::env;

use biddy::{service, cli};

pub fn main() {
    let usage = "--serve or --cli-args";
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() == 0 {
        println!("{}", usage);
    } else if args[0] == "--serve" {
        println!("serving...");
        service::start();
    } else {
        cli::consume(args);
    }

}
