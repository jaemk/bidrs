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
//mod login;
//mod logout;
//mod msg;
//mod users;
//mod whoami;


/// handler prelude of imports needed by handlers, so handler mods
/// can just do a:
/// ```rust,ignore
/// use super::prelude::*;
/// ```
mod prelude {
    // iron stuff
    pub use iron::{Handler, Request, Response, IronResult, status, headers};

    // our libs
    pub use auth;
    pub use sessions::{Session, SessionStore};

    // local types
    pub use super::PgPool;
    pub use super::SStore;

    /// Return an unauthorized response, optionally specify the message
    pub fn unauthorized(message: Option<String>) -> IronResult<Response> {
        let _msg = match message {
            Some(m) => m,
            None => "invalid credentials".to_string(),
        };
        let msg = object!{"msg" => _msg};
        return Ok(Response::with((status::Unauthorized, msg.dump())))
    }
}

/// Handler Manager
///
/// Initializes all handlers with any external resources they need
pub struct Handlers {
    pub hello: hello::HelloHandler,
    //pub login: login::LoginHandler,
    //pub logout: logout::LogoutHandler,
    //pub users: users::UsersHandler,
    //pub post_msg: msg::PostMsgHandler,
    //pub get_msg: msg::GetMsgHandler,
    //pub whoami: whoami::WhoamiHandler,
}
impl Handlers {
    pub fn new(db_pool: PgPool, s_store: SStore) -> Handlers {
        Handlers {
            hello: hello::HelloHandler::new(),
            //post_msg: msg::PostMsgHandler::new(),
            //get_msg: msg::GetMsgHandler::new(),
            //login: login::LoginHandler::new(db_pool.clone(), s_store.clone()),
            //logout: logout::LogoutHandler::new(s_store.clone()),
            //users: users::UsersHandler::new(db_pool.clone()),
            //whoami: whoami::WhoamiHandler::new(db_pool.clone(), s_store.clone()),
        }
    }
}

