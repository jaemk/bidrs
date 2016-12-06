//! Login handler
//!
//! Accepts unauthenticated POSTs with json data containing
//! an 'email' and 'password' to be authenticated.
//! Returns a new auth/session token on success.
use std::io::Read;
use super::prelude::*;


#[derive(RustcEncodable, RustcDecodable)]
struct ApiAuth {
    email: String,
    password: String,
}
#[derive(Debug, RustcEncodable, RustcDecodable)]
struct AuthSuccess {
    token: String,
    admin: bool,
}
pub struct LoginHandler {
    db_pool: PgPool,
    s_store: SStore,
}
impl LoginHandler {
    pub fn new(db_pool: PgPool, s_store: SStore) -> LoginHandler {
        LoginHandler {
            db_pool: db_pool,
            s_store: s_store,
        }
    }
}
impl Handler for LoginHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        // get post info
        let mut req_body = String::new();
        request.body.read_to_string(&mut req_body).unwrap();
        let auth_info: ApiAuth = try_server_error!(json::decode(&req_body));

        // look for user by email
        let conn = self.db_pool.get().unwrap();
        let user = match sql::select_authuser_by_email(&conn, &auth_info.email) {
            Some(u) => u,
            None => return unauthorized(None),
        };

        // hash the provided password with the found-user's salt
        let hash = match auth::hash(auth_info.password.as_str(), &user.salt) {
            Ok(h) => h,
            _ => return unauthorized(None),
        };

        if !auth::const_eq(hash.as_slice(), user.password.as_slice()) {
            return unauthorized(None);
        }

        // auth was successful, initialize a new session
        let new_sess = Session::new(&user.uuid);
        let resp = AuthSuccess {
            token: new_sess.token.clone(),
            admin: user.level > 9,
        };
        self.s_store.lock().unwrap().add(new_sess);

        // send back the new auth/session token
        Ok(Response::with((status::Ok, json::encode(&resp).unwrap())))
    }
}
