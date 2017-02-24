#![recursion_limit = "1024"]

extern crate chrono;
extern crate uuid;

extern crate rustc_serialize;
#[macro_use]
extern crate json;

extern crate jsonwebtoken as jwt;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate env_logger;
extern crate dotenv;
extern crate rpassword;

extern crate crypto;
extern crate rand;

#[macro_use]
extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

// server specific
extern crate iron;
extern crate router;
extern crate plugin;
extern crate logger;
extern crate mount;
extern crate staticfile;

#[macro_use]
extern crate error_chain;
pub mod errors {
    error_chain! { }
}

#[macro_use]
pub mod macros;

pub mod models;
pub mod sql;
pub mod auth;
pub mod sessions;
pub mod service;
pub mod handlers;
pub mod middleware;
pub mod cli;
