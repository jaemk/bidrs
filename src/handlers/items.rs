//! Items Handlers
//!
use super::prelude::*;

#[derive(RustcEncodable)]
struct Items {
    items: Vec<models::Item>
}

pub struct ItemsHandler {
    db_pool: PgPool,
    s_store: SStore,
}
impl ItemsHandler {
    pub fn new(db_pool: PgPool, s_store: SStore) -> ItemsHandler {
        ItemsHandler { db_pool: db_pool, s_store: s_store }
    }
}
impl Handler for ItemsHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let conn = self.db_pool.get().unwrap();
        let store = self.s_store.lock().unwrap();
        let user = store.get_user_from_request(&conn, &request).unwrap();

        let items = sql::filter_items_for_user(&conn, user.id);
        let items = Items { items: items };
        Ok(Response::with((status::Ok, json::encode(&items).unwrap())))
    }
}
