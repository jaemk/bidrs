//! Handlers
//!
//! Handlers manager & handler-prelude
//!
use std::sync::{Arc, Mutex};

use r2d2::Pool;
use r2d2_postgres::PostgresConnectionManager;

use sessions::SessionStore;

pub type PgPool = Pool<PostgresConnectionManager>;
pub type SStore = Arc<Mutex<SessionStore>>;

// handler defs
mod hello;
mod info;
mod login;
mod logout;

mod items;


/// handler prelude of imports needed by handlers, so handler mods
/// can just do a:
/// ```rust,ignore
/// use super::prelude::*;
/// ```
mod prelude {
    // iron stuff
    pub use iron::{Handler, Request, Response, IronResult, status, headers};
    pub use iron::modifiers::Redirect;
    pub use rustc_serialize::json;

    // our libs
    pub use sql;
    pub use models;
    pub use auth;
    pub use sessions::{Session, SessionStore};

    // local types
    pub use super::PgPool;
    pub use super::SStore;

    #[derive(Debug, RustcEncodable)]
    pub struct Msg { pub msg: String }

    /// Return an unauthorized response, optionally specify the message
    pub fn unauthorized(message: Option<String>) -> IronResult<Response> {
        let _msg = match message {
            Some(m) => m,
            None => "invalid credentials".to_string(),
        };
        let msg = Msg { msg: _msg };
        return Ok(Response::with((status::Unauthorized, json::encode(&msg).unwrap())))
    }
}

/// Handler Manager
///
/// Initializes all handlers with any external resources they need
pub struct Handlers {
    pub hello: hello::HelloHandler,
    pub info: info::InfoHandler,
    pub login: login::LoginHandler,
    pub logout: logout::LogoutHandler,
    pub items: items::ItemsHandler,
}
impl Handlers {
    pub fn new(db_pool: PgPool, s_store: SStore) -> Handlers {
        Handlers {
            hello: hello::HelloHandler::new(),
            info: info::InfoHandler::new(db_pool.clone(), s_store.clone()),
            login: login::LoginHandler::new(db_pool.clone(), s_store.clone()),
            logout: logout::LogoutHandler::new(s_store.clone()),
            items: items::ItemsHandler::new(db_pool.clone(), s_store.clone()),
        }
    }
}

