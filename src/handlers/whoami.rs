//! Who am i
//!
//! Return the current user info from the current session
use super::prelude::*;

pub struct WhoamiHandler {
    pool: PgPool,
    store: SStore,
}
impl WhoamiHandler {
    pub fn new(db_pool: PgPool, s_store: SStore) -> WhoamiHandler {
        WhoamiHandler { store: s_store , pool: db_pool }
    }
}
impl Handler for WhoamiHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let mut store = self.store.lock().unwrap();
        let session = store.get_mut_from_request(&request);

        let uuid = session.unwrap().user_uuid;
        let conn = self.pool.get().unwrap();
        let user: sql::models::UserShort = sql::select_user_by_uuid(&conn, &uuid).unwrap();
        Ok(Response::with((status::Ok, json::encode(&user).unwrap())))
    }
}
