use postgres::Connection;
use uuid::Uuid;
use models::*;


pub fn select_user_by_email(conn: &Connection, email: &str) -> Option<User> {
    let qs = "select * from users where email=$1";
    query_or_none!(conn.query(qs, &[&email]), User)
}

pub fn select_user_by_uuid(conn: &Connection, uuid_: &Uuid) -> Option<User> {
    let qs = "select * from users where uuid_=$1 limit 1";
    query_or_none!(conn.query(qs, &[&uuid_]), User)
}

pub fn select_profile_by_user(conn: &Connection, user_id: &i32) -> Option<Profile> {
    let qs = "select * from profiles where user_id=$1 limit 1";
    query_or_none!(conn.query(qs, &[&user_id]), Profile)
}


