extern crate bidrs;

use std::env;

use bidrs::{service, cli};

pub fn main() {
    let usage = "serve or --cli-args";
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.len() == 0 {
        println!("{}", usage);
    } else if args[0] == "serve" {
        let quiet = args.len() != 1 && args[1] == "quiet";
        println!("serving...");
        service::start(quiet);
    } else {
        cli::consume(args);
    }

}
