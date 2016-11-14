extern crate postgres;
extern crate chrono;
extern crate uuid;

use self::postgres::{Connection};
use self::uuid::Uuid;

pub mod models;

pub fn select_user_by_id(conn: &Connection, user_id: &i32) -> Option<models::User> {
    for row in conn.query("select id, username, uuid_, date_created, date_modified from biddy_user where id = $1", &[user_id]).unwrap().iter() {
        let user = models::User {
            id: row.get(0),
            username: row.get(1),
            uuid: row.get(2),
            date_created: row.get(3),
            date_modified: row.get(4),
        };
        return Some(user);
    }
    return None;
}

pub fn select_user_latest(conn: &Connection) -> Option<models::User> {
    for row in conn.query("select id, username, uuid_, date_created, date_modified from biddy_user order by date_created desc limit 1", &[]).unwrap().iter() {
        let user = models::User {
            id: row.get(0),
            username: row.get(1),
            uuid: row.get(2),
            date_created: row.get(3),
            date_modified: row.get(4),
        };
        return Some(user);
    }
    return None;
}

pub fn insert_user(conn: Connection, username: String) {
    conn.execute("insert into biddy_user (username, uuid_) values ($1, $2)",
                 &[&username, &Uuid::new_v4()]).unwrap();
}
