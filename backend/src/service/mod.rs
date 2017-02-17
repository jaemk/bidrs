//! Service
//!
//! Server, router, external resources setup
//!
mod routes;

use std::sync::{Arc, Mutex};

use std::path::Path;
use mount::Mount;
use staticfile::Static;

use iron::prelude::*;
use router::Router;
use env_logger;
use logger::Logger;

use std::env;
use dotenv::dotenv;
use r2d2::{Config, Pool};
use r2d2_postgres::{PostgresConnectionManager, TlsMode as r2d2TlsMode};
use postgres::{Connection, TlsMode as PgTlsMode};

use handlers::{Handlers};
use middleware::{InfoLog, SessionMiddleware};
use sessions::{self, SessionStore};


/// Create a new postgres database connection
pub fn establish_connection() -> Connection {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Connection::connect(db_url.as_ref(), PgTlsMode::None)
        .expect(&format!("Error connecting to {}", db_url))
}


/// Create a new r2d2 pool of postgres connections
pub fn establish_pool_connection() -> Pool<PostgresConnectionManager> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_mgr = PostgresConnectionManager::new(db_url, r2d2TlsMode::None)
        .expect("Connection fail");
    Pool::new(Config::default(), db_mgr).expect("Failed to create pool")
}


/// Start up the server
pub fn start(host: &str, quiet: bool) {
    // setup db connection pool
    let db_pool = establish_pool_connection();
    println!(">> Connected to db!");

    // setup session store access, exempt url roots, and store-cleaning daemon
    let session_store = Arc::new(Mutex::new(SessionStore::new(20 * 60)));
    let exempt_url_roots = hashset!("login", "hello");
    let session_middleware = SessionMiddleware::new(session_store.clone(), exempt_url_roots);
    sessions::start_daemon_sweeper(session_store.clone(), 30 * 60);
    println!(">> Session store created");

    // setup general loggers
    env_logger::init().unwrap();
    let (log_before, log_after) = Logger::new(None);

    // initialize handler manager with external resources
    let handlers = Handlers::new(db_pool, session_store);

    // Setup endpoints
    let mut router = Router::new();
    routes::mount(&mut router, handlers);

    // Add middleware
    let mut chain = Chain::new(router);
    if !quiet {
        chain.link_before(log_before);      // general logger
        chain.link_before(InfoLog);         // custom request-info log
        chain.link_after(log_after);        // general logger
    }
    chain.link_around(session_middleware);  // custom session middleware

    let mut mount = Mount::new();
    mount.mount("/", chain).mount("/static/", Static::new(Path::new("../static")));

    println!(">> Serving at {}", host);
    if quiet { println!(">> ... quietly") }
    Iron::new(mount).http(host).unwrap();
}
