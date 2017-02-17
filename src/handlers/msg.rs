//! Msg handlers
//!
use std::io::Read;
use std::collections::HashMap;
use super::prelude::*;


/// PostMsgHandler
///
pub struct PostMsgHandler;
impl PostMsgHandler {
    pub fn new() -> PostMsgHandler {
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


/// Get msg handler
///
pub struct GetMsgHandler;
impl GetMsgHandler {
    pub fn new() -> GetMsgHandler {
        GetMsgHandler {}
    }
}
impl Handler for GetMsgHandler {
    fn handle(&self, _: &mut Request) -> IronResult<Response> {
        let mut msg = HashMap::new();
        msg.insert("msg".to_string(), "hello!".to_string());
        Ok(Response::with((status::Ok, json::encode(&msg).unwrap())))
    }
}


