//! Users handlers
//!
use super::prelude::*;


pub struct UsersHandler {
    db_pool: PgPool,
}
impl UsersHandler {
    pub fn new(pool: PgPool) -> UsersHandler {
        UsersHandler {
            db_pool: pool,
        }
    }
}
impl Handler for UsersHandler {
    fn handle(&self, _request: &mut Request) -> IronResult<Response> {
        let conn = self.db_pool.get().unwrap();
        let users = sql::select_users_all(&conn);
        let payload = json::encode(&users).unwrap();
        Ok(Response::with((status::Ok, payload)))
    }
}

