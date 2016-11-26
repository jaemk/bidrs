use super::iron::prelude::*;
use super::iron::status;
use super::router::Router;
use super::env_logger;
use super::logger::Logger;

use super::rustc_serialize::json;

use super::r2d2::{Config, Pool};
use super::r2d2_postgres::{PostgresConnectionManager, TlsMode};

use super::sql;
use super::handlers::{Handlers};
use super::middleware::{InfoLog};

pub fn start() {
    let db_url = "postgresql://biddy:biddy@localhost";
    let db_mgr = PostgresConnectionManager::new(db_url, TlsMode::None).expect("connection fail");
    let pool = Pool::new(Config::default(), db_mgr).expect("pool fail");
    println!("connected to db!");

    env_logger::init().unwrap();
    let (log_before, log_after) = Logger::new(None);

    let handlers = Handlers::new(pool);

    let mut router = Router::new();

    router.get("/hello", handlers.hello, "hello");
    router.get("/users", handlers.users, "users");
    router.post("/msg", handlers.post_msg , "post_msg");

    let mut chain = Chain::new(router);
    chain.link_before(log_before);
    chain.link_before(InfoLog);
    chain.link_after(log_after);

    Iron::new(chain).http("localhost:8000").unwrap();
}
