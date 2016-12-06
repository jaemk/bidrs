//! Service
//!
//! Server, router, external resources setup
//!
use std::sync::{Arc, Mutex};

use super::iron::prelude::*;
use super::router::Router;
use super::env_logger;
use super::logger::Logger;

use super::r2d2::{Config, Pool};
use super::r2d2_postgres::{PostgresConnectionManager, TlsMode};

use super::handlers::{Handlers};
use super::middleware::{InfoLog, SessionMiddleware};
use super::sessions::{self, SessionStore};

pub fn start() {
    // setup db connection pool
    let db_url = "postgresql://biddy:biddy@localhost";
    let db_mgr = PostgresConnectionManager::new(db_url, TlsMode::None).expect("connection fail");
    let db_pool = Pool::new(Config::default(), db_mgr).expect("pool fail");
    println!(">> Connected to db!");

    // setup session store access and daemon
    let session_store = Arc::new(Mutex::new(SessionStore::new(20 * 60)));
    let session_middleware = SessionMiddleware::new(session_store.clone());
    sessions::start_daemon_sweeper(session_store.clone(), 30 * 60);
    println!(">> Session store created");

    // setup general loggers
    env_logger::init().unwrap();
    let (log_before, log_after) = Logger::new(None);

    // initialize handler manager with external resources
    let handlers = Handlers::new(db_pool, session_store);

    // Setup endpoints
    let mut router = Router::new();
    router.post("/login", handlers.login, "login");
    router.post("/logout", handlers.logout, "logout");
    router.get("/hello", handlers.hello, "hello");
    router.get("/users", handlers.users, "users");
    router.post("/msg", handlers.post_msg , "post_msg");
    router.get("/msg", handlers.get_msg, "get_msg");
    router.get("/whoami", handlers.whoami, "whoami");

    // Add middleware
    let mut chain = Chain::new(router);
    chain.link_before(log_before);          // general logger
    chain.link_before(InfoLog);             // custom request-info log
    chain.link_around(session_middleware);  // custom session middleware
    chain.link_after(log_after);            // general logger

    let host = "localhost:8000";
    println!(">> Serving at {}", host);
    Iron::new(chain).http(host).unwrap();
}
