/// Handlers
///
/// Handlers manager & handler impls

use std::io::Read;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::r2d2::Pool;
use super::r2d2_postgres::PostgresConnectionManager;
use super::iron::{Handler, Request, Response, IronResult, status};
use super::rustc_serialize::json;
use super::uuid::Uuid;

use super::sql;
use super::sessions::{Session, SessionStore, SessionKey};

type PgPool = Pool<PostgresConnectionManager>;
type SStore = Arc<Mutex<SessionStore>>;

#[derive(RustcEncodable, RustcDecodable)]
struct Msg {
    msg: String,
}

/// All handlers
///
pub struct Handlers {
    pub hello: HelloHandler,
    pub login: LoginHandler,
    pub users: UsersHandler,
    pub post_msg: PostMsgHandler,
}
impl Handlers {
    pub fn new(db_pool: PgPool, s_store: SStore) -> Handlers {
        Handlers {
            hello: HelloHandler::new(),
            post_msg: PostMsgHandler::new(),
            login: LoginHandler::new(db_pool.clone(), s_store.clone()),
            users: UsersHandler::new(db_pool.clone()),
        }
    }
}

/// Hello
///
pub struct HelloHandler;
impl HelloHandler {
    fn new() -> HelloHandler {
        HelloHandler {}
    }
}
impl Handler for HelloHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "hello!")))
    }
}


/// PostMsgHandler
///
pub struct PostMsgHandler;
impl PostMsgHandler {
    fn new() -> PostMsgHandler {
        PostMsgHandler {}
    }
}
impl Handler for PostMsgHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let mut req_body = String::new();
        request.body.read_to_string(&mut req_body).unwrap();
        let msg: HashMap<String, String> = try_server_error!(json::decode(&req_body));
        println!("got: {:?}", msg.get(&"msg".to_string()));
        Ok(Response::with((status::Ok, json::encode(&msg).unwrap())))
    }
}


/// Login
///
#[derive(RustcEncodable, RustcDecodable)]
struct Auth {
    username: String,
    password: String,
}
#[derive(Debug, RustcEncodable, RustcDecodable)]
struct Token {
    token: String,
}
pub struct LoginHandler {
    db_pool: PgPool,
    s_store: SStore,
}
impl LoginHandler {
    fn new(db_pool: PgPool, s_store: SStore) -> LoginHandler {
        LoginHandler {
            db_pool: db_pool,
            s_store: s_store,
        }
    }
}
impl Handler for LoginHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let mut req_body = String::new();
        request.body.read_to_string(&mut req_body).unwrap();
        let auth: Auth = try_server_error!(json::decode(&req_body));
        if auth.username != "admin" && auth.password != "admin" {
            let msg = Msg { msg: "invalid credentials".to_string() };
            return Ok(Response::with((status::Unauthorized, json::encode(&msg).unwrap())))
        }
        let uuid = Uuid::new_v4(); // in the future this should be the user's actual uuid
        let new_sess = Session::new(&uuid);
        let token = Token { token: new_sess.token.clone() };
        self.s_store.lock().unwrap().add(new_sess);
        Ok(Response::with((status::Ok, json::encode(&token).unwrap())))
    }
}


/// Users
///
pub struct UsersHandler {
    db_pool: PgPool,
}
impl UsersHandler {
    fn new(pool: PgPool) -> UsersHandler {
        UsersHandler {
            db_pool: pool,
        }
    }
}
impl Handler for UsersHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        println!("request-session-key: {:?}", request.extensions.get::<SessionKey>());
        let conn = self.db_pool.get().unwrap();
        let users = sql::select_users_all(&conn);
        let payload = json::encode(&users).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }
}

