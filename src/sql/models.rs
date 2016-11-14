extern crate chrono;
extern crate uuid;
extern crate rustc_serialize;

#[derive(Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub uuid: uuid::Uuid,
    pub date_created: chrono::DateTime<chrono::UTC>,
    pub date_modified: chrono::DateTime<chrono::UTC>,
}

pub struct Organization {
    id: i32,
    name: String,
    extra: rustc_serialize::json::Json,
}
