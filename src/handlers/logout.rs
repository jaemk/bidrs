//! Logout Handler
//!
//! Removes an authenticated session by the request's Authorization key
use super::prelude::*;


pub struct LogoutHandler {
    store: SStore,
}
impl LogoutHandler {
    pub fn new(s_store: SStore) -> LogoutHandler {
        LogoutHandler {
            store: s_store,
        }
    }
}
impl Handler for LogoutHandler {
    fn handle(&self, request: &mut Request) -> IronResult<Response> {
        let mut store = self.store.lock().unwrap();
        store.delete_by_request(request);
        let msg = object!{ "msg" => "logout".to_string() };
        Ok(Response::with((status::Ok, msg.dump())))
    }
}
