extern crate biddy;
extern crate postgres;

use std::env;
use postgres::{Connection, TlsMode};

pub fn main() {
    let arg = env::args().nth(1).unwrap_or("select".to_string());

    let conn = Connection::connect("postgresql://biddy:biddy@localhost", TlsMode::None).unwrap();
    println!("connected");

    if arg == "select" {
        let id = env::args().nth(2).unwrap_or("1".to_string());
        let idint = id.parse::<i32>().unwrap_or(1);
        let user = biddy::sql::select_user_by_id(&conn, &idint);
        println!("{:?}", user);
    } else if arg == "latest" {
        let user = biddy::sql::select_user_latest(&conn);
        println!("{:?}", user);
    } else if arg == "insert" {
        let username = env::args().nth(2).unwrap_or("james".to_string());
        biddy::sql::insert_user(conn, username);
    }
}
