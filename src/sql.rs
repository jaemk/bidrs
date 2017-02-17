use postgres::Connection;
use models::*;


pub fn select_user_by_email(conn: &Connection, email: &str) -> Option<User> {
    let qs = "select * from users where email=$1";
    query_or_none!(conn.query(qs, &[&email]), User)
}


pub fn user_level_by_userid(conn:&Connection, id: &i32) -> Option<i32> {
    let qs = "select level_ from profiles where user_id=$1";
    return match conn.query(qs, &[&id]).unwrap().iter().next() {
        Some(row) => Some(row.get(0)),
        _ => None
    }
}
