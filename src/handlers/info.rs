//! Who am i
//!
//! Return the current user info from the current session
use super::prelude::*;
use self::json::Json;
use uuid::Uuid;

pub struct InfoHandler {
    pool: PgPool,
    store: SStore,
}
impl InfoHandler {
    pub fn new(db_pool: PgPool, s_store: SStore) -> InfoHandler {
        InfoHandler { store: s_store , pool: db_pool }
    }
}


#[derive(RustcEncodable)]
struct Info {
    email: String,
    uuid_: Uuid,
    level_: i32,
    is_primary: bool,
    name: String,
    phone: Option<String>,
    extra: Option<Json>,
}

impl Handler for InfoHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let mut store = self.store.lock().unwrap();
        let session = store.get_mut_from_request(&request);

        let uuid = session.unwrap().user_uuid;
        let conn = self.pool.get().unwrap();
        let user = sql::select_user_by_uuid(&conn, &uuid).unwrap();
        let profile = sql::select_profile_by_user(&conn, &user.id).unwrap();

        let info = Info {
            email: user.email,
            uuid_: user.uuid_,
            level_: user.level_,
            is_primary: profile.is_primary,
            name: profile.name,
            phone: profile.phone,
            extra: profile.extra,
        };
        Ok(Response::with((status::Ok, json::encode(&info).unwrap())))
    }
}
