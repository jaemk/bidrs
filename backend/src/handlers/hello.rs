//! Hello handler
//!
use super::prelude::*;


pub struct HelloHandler;
impl HelloHandler {
    pub fn new() -> HelloHandler {
        HelloHandler {}
    }
}
impl Handler for HelloHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        Ok(Response::with((status::Ok, "{\"data\": \"hello!\"}")))
    }
}
