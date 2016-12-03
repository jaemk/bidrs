//! Handlers
//!
//! Handlers manager & handler-prelude
//!
use std::sync::{Arc, Mutex};

use super::r2d2::Pool;
use super::r2d2_postgres::PostgresConnectionManager;

use super::sessions::SessionStore;

pub type PgPool = Pool<PostgresConnectionManager>;
pub type SStore = Arc<Mutex<SessionStore>>;

// handler defs
mod login;
mod hello;
mod msg;
mod users;
mod whoami;


#[derive(RustcEncodable, RustcDecodable)]
/// Generic response message to be encoded as json
pub struct Msg {
    msg: String,
}

/// handler prelude of imports needed by handlers, so handler mods
/// can just do a:
/// ```rust,ignore
/// use super::prelude::*;
/// ```
mod prelude {
    // iron stuff
    pub use super::super::iron::{Handler, Request, Response, IronResult, status, headers};

    // extern crate stuff
    pub use super::super::rustc_serialize::json;
    pub use super::super::uuid::Uuid;

    // our libs
    pub use super::super::sql;
    pub use super::super::auth;
    pub use super::super::sessions::{Session, SessionStore};

    // local types
    pub use super::Msg;
    pub use super::PgPool;
    pub use super::SStore;

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
    pub login: login::LoginHandler,
    pub users: users::UsersHandler,
    pub post_msg: msg::PostMsgHandler,
    pub get_msg: msg::GetMsgHandler,
    pub whoami: whoami::WhoamiHandler,
}
impl Handlers {
    pub fn new(db_pool: PgPool, s_store: SStore) -> Handlers {
        Handlers {
            hello: hello::HelloHandler::new(),
            post_msg: msg::PostMsgHandler::new(),
            get_msg: msg::GetMsgHandler::new(),
            login: login::LoginHandler::new(db_pool.clone(), s_store.clone()),
            users: users::UsersHandler::new(db_pool.clone()),
            whoami: whoami::WhoamiHandler::new(db_pool.clone(), s_store.clone()),
        }
    }
}

