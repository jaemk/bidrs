extern crate postgres;
extern crate chrono;
extern crate uuid;
extern crate rustc_serialize;

use self::postgres::{Connection};
use self::uuid::Uuid;
use self::rustc_serialize::json;

pub mod models;

// User related queries
//
pub fn select_user_by_id(conn: &Connection, user_id: &i32) -> Option<models::User> {
    let qs = "select id, username, uuid_, date_created, date_modified \
              from biddy_user where id = $1";
    match conn.query(qs, &[user_id]).unwrap().iter().next() {
        Some(row) => Some(models::User::from_row(row)),
        _ => None,
    }
}
pub fn select_user_latest(conn: &Connection) -> Option<models::User> {
    let qs = "select id, username, uuid_, date_created, date_modified \
              from biddy_user order by date_created desc limit 1";
    match conn.query(qs, &[]).unwrap().iter().next() {
        Some(row) => Some(models::User::from_row(row)),
        _ => None,
    }
}
pub fn select_users_all(conn: &Connection) -> Vec<models::User> {
    let qs = "select id, username, uuid_, date_created, date_modified \
              from biddy_user";
    conn.query(qs, &[]).unwrap().iter().map(|row| models::User::from_row(row)).collect::<Vec<_>>()
}
pub fn insert_user(conn: Connection, username: &String) -> Result<u64, postgres::error::Error> {
    let res = try!(conn.execute("insert into biddy_user (username, uuid_) values ($1, $2)",
                   &[username, &Uuid::new_v4()]));
    Ok(res)
}

// Organization related queries
//
pub fn select_org_by_id(conn: &Connection, org_id: &i32) -> Option<models::Organization> {
    let qs = "select id, name, extra, date_created, date_modified \
              from organization where id = $1";
    match conn.query(qs, &[org_id]).unwrap().iter().next() {
        Some(row) => Some(models::Organization::from_row(row)),
        _ => None,
    }
}
pub fn select_org_latest(conn: &Connection) -> Option<models::Organization> {
    let qs = "select id, name, extra, date_created, date_modified \
              from organization order by date_created desc limit 1";
    match conn.query(qs, &[]).unwrap().iter().next() {
        Some(row) => Some(models::Organization::from_row(row)),
        _ => None,
    }
}
pub fn select_orgs_all(conn: &Connection) -> Vec<models::Organization> {
    let qs = "select id, name, extra, date_created, date_modified from organization";
    conn.query(qs, &[]).unwrap().iter().map(|row| models::Organization::from_row(row)).collect::<Vec<_>>()
}
pub fn insert_org(conn: Connection, name: &String, extra: &json::Json) -> Result<u64, postgres::error::Error> {
    let qs = "insert into organization (name, extra) values ($1, $2)";
    let res = try!(conn.execute(qs, &[name, extra]));
    Ok(res)
}

// Bidder related queries
//
pub fn select_bidder_by_id(conn: &Connection, bidder_id: &i32) -> Option<models::Bidder> {
    let qs = "select id, organization_id, date_created, date_modified \
              from bidder where id = $1";
    match conn.query(qs, &[bidder_id]).unwrap().iter().next() {
        Some(row) => Some(models::Bidder::from_row(row)),
        _ => None,
    }
}
pub fn select_bidder_latest(conn: &Connection) -> Option<models::Bidder> {
    let qs = "select id, organization_id, date_created, date_modified \
              from bidder order by date_created desc limit 1";
    match conn.query(qs, &[]).unwrap().iter().next() {
        Some(row) => Some(models::Bidder::from_row(row)),
        _ => None,
    }
}
pub fn select_bidders_all(conn: &Connection) -> Vec<models::Bidder> {
    let qs = "select id, organization_id, date_created, date_modified from bidder";
    conn.query(qs, &[]).unwrap().iter().map(|row| models::Bidder::from_row(row)).collect::<Vec<_>>()
}
pub fn select_bidders_by_org(conn: &Connection, org_id: &i32) -> Vec<models::Bidder> {
    let qs = "select id, organization_id, date_created, date_modified \
              from bidder where organization_id = $1";
    conn.query(qs, &[org_id]).unwrap().iter().map(|row| models::Bidder::from_row(row)).collect::<Vec<_>>()
}
pub fn insert_bidder(conn: Connection, org_id: &i32) -> Result<u64, postgres::error::Error> {
    let qs = "insert into bidder (organization_id) values ($1)";
    let res = try!(conn.execute(qs, &[org_id]));
    Ok(res)
}

// Item related queries
//
pub fn select_items_all(conn: &Connection) -> Vec<models::Item> {
    let qs = "select id, organization_id, owning_bidder_id, is_goal, title, \
              description, value, min_bid, date_created, date_modified \
              from item";
    conn.query(qs, &[]).unwrap().iter().map(|row| models::Item::from_row(row)).collect::<Vec<_>>()
}
pub fn insert_item(conn: Connection, org_id: &i32, is_goal: &bool,
                   title: &String, desc: &String, value: &i64, min_bid: &i64)
    -> Result<u64, postgres::error::Error> {
    let qs = "insert into item (organization_id, is_goal, title, description, value, min_bid) \
              values ($1, $2, $3, $4, $5, $6)";
    let res = try!(conn.execute(qs, &[org_id, is_goal, title, desc, value, min_bid]));
    Ok(res)
}

