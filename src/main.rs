extern crate bidrs;
extern crate clap;

use bidrs::{cli, service};
use bidrs::errors::*;
use clap::{Arg, App, SubCommand, ArgMatches};


pub fn main() {
    let matches = App::new("bidrs")
        .version("0.1.0")
        .author("James K. <james.kominick@gmail.com>")
        .about("")
        .subcommand(SubCommand::with_name("serve")
                    .about("Run server")
                    .arg(Arg::with_name("port")
                         .long("port")
                         .short("p")
                         .help("Port to listen on. Defaults to 3002"))
                    .arg(Arg::with_name("private")
                         .long("private")
                         .help("only make available on localhost"))
                    .arg(Arg::with_name("quiet")
                         .long("quiet")
                         .short("q")
                         .help("Don't print any logs.")))
        .subcommand(SubCommand::with_name("cli")
                    .about("CLI functions")
                    .arg(Arg::with_name("create-all")
                         .long("create-all")
                         .help("create user, org, bidder, \
                                payment-info, profile, item, bid"))
                    .arg(Arg::with_name("create-user")
                         .long("create-user")
                         .help("create new user"))
                    .arg(Arg::with_name("create-org")
                         .long("create-org")
                         .help("create new organization"))
                    .arg(Arg::with_name("create-bidder")
                         .long("create-bidder")
                         .help("create new bidder"))
                    .arg(Arg::with_name("create-payment-info")
                         .long("create-payment-info")
                         .help("create new payment info"))
                    .arg(Arg::with_name("create-profile")
                         .long("create-profile")
                         .help("create new profile"))
                    .arg(Arg::with_name("create-item")
                         .long("create-item")
                         .help("create new item"))
                    .arg(Arg::with_name("create-bid")
                         .long("create-bid")
                         .help("create new bid")))
        .get_matches();

    if let Err(ref e) = run(matches) {
        use ::std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let stderr_msg = "Error writing to stderr";
        writeln!(stderr, "error: {}", e).expect(stderr_msg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(stderr_msg);
        }

        // `RUST_BACKTRACE=1`
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(stderr_msg);
        }

        ::std::process::exit(1);
    }
}


/// Handle all arg-matches
fn run(matches: ArgMatches) -> Result<()> {
    // startup our server
    if let Some(serve_matches) = matches.subcommand_matches("serve") {
        let ip = if serve_matches.is_present("private") {
            "127.0.0.1"
        } else {
            "0.0.0.0"
        };
        let port = serve_matches.value_of("port").unwrap_or("3002");
        let host = format!("{}:{}", ip, port);
        let quiet = matches.is_present("quiet");
        service::start(&host, quiet);
        return Ok(());
    }

    // handle cli stuff
    if let Some(cli_matches) = matches.subcommand_matches("cli") {
        if cli_matches.is_present("create-user") || cli_matches.is_present("create-all") {
            cli::create_user()
                .chain_err(|| "Error creating new user")?;
        }

        if cli_matches.is_present("create-org") || cli_matches.is_present("create-all") {
            cli::create_org()
                .chain_err(|| "Error creating new org")?;
        }

        if cli_matches.is_present("create-bidder") || cli_matches.is_present("create-all") {
            cli::create_bidder()
                .chain_err(|| "Error creating new bidder")?;
        }

        if cli_matches.is_present("create-payment-info") || cli_matches.is_present("create-all") {
            cli::create_payment_info()
                .chain_err(|| "Error creating new payment info")?;
        }

        if cli_matches.is_present("create-profile") || cli_matches.is_present("create-all") {
            cli::create_profile()
                .chain_err(|| "Error creating new profile")?;
        }

        if cli_matches.is_present("create-item") || cli_matches.is_present("create-all") {
            cli::create_item()
                .chain_err(|| "Error creating new item")?;
        }

        if cli_matches.is_present("create-bid") || cli_matches.is_present("create-all") {
            cli::create_bid()
                .chain_err(|| "Error creating new bid")?;
        }

    }

    Ok(())
}
