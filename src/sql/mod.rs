
use super::postgres::{self, Connection};
use super::uuid::Uuid;
use super::rustc_serialize::json;

pub mod models;

use self::models::{
    User, Organization,
    Bidder, Item,
    Profile, Bid,
};

// User related queries
//
pub fn select_user_by_id(conn: &Connection, user_id: &i32) -> Option<User> {
    let qs = "select id, username, uuid_, date_created, date_modified \
              from biddy_user where id = $1";
    query_or_none!(conn.query(qs, &[user_id]), User)
}
pub fn select_user_latest(conn: &Connection) -> Option<User> {
    let qs = "select id, username, uuid_, date_created, date_modified \
              from biddy_user order by date_created desc limit 1";
    query_or_none!(conn.query(qs, &[]), User)
}
pub fn select_users_all(conn: &Connection) -> Vec<User> {
    let qs = "select id, username, uuid_, date_created, date_modified \
              from biddy_user";
    query_coll!(conn.query(qs, &[]), User)
}
pub fn insert_user(conn: Connection, username: String) -> Result<User, String> {
    let qs = "insert into biddy_user (username, uuid_) values ($1, $2) \
              returning id, date_created, date_modified";
    let uuid = Uuid::new_v4();
    try_insert_to_model!(conn.query(qs, &[&username, &uuid]) ;
                         User ;
                         id: 0, date_created: 1, date_modified: 2 ;
                         username: username, uuid: uuid)
}

// Organization related queries
//
pub fn select_org_by_id(conn: &Connection, org_id: &i32) -> Option<Organization> {
    let qs = "select id, name, extra, date_created, date_modified \
              from organization where id = $1";
    query_or_none!(conn.query(qs, &[org_id]), Organization)
}
pub fn select_org_latest(conn: &Connection) -> Option<Organization> {
    let qs = "select id, name, extra, date_created, date_modified \
              from organization order by date_created desc limit 1";
    query_or_none!(conn.query(qs, &[]), Organization)
}
pub fn select_orgs_all(conn: &Connection) -> Vec<Organization> {
    let qs = "select id, name, extra, date_created, date_modified from organization";
    query_coll!(conn.query(qs, &[]), Organization)
}
pub fn insert_org(conn: Connection, name: String, extra: Option<json::Json>) -> Result<Organization, String> {
    let qs = "insert into organization (name, extra) values ($1, $2) \
              returning id, date_created, date_modified";
    try_insert_to_model!(conn.query(qs, &[&name, &extra]) ;
                         Organization ;
                         id: 0, date_created: 1, date_modified: 2 ;
                         name: name, extra: extra)
}

// Bidder related queries
//
pub fn select_bidder_by_id(conn: &Connection, bidder_id: &i32) -> Option<Bidder> {
    let qs = "select id, organization_id, date_created, date_modified \
              from bidder where id = $1";
    query_or_none!(conn.query(qs, &[bidder_id]), Bidder)
}
pub fn select_bidder_latest(conn: &Connection) -> Option<Bidder> {
    let qs = "select id, organization_id, date_created, date_modified \
              from bidder order by date_created desc limit 1";
    query_or_none!(conn.query(qs, &[]), Bidder)
}
pub fn select_bidders_all(conn: &Connection) -> Vec<Bidder> {
    let qs = "select id, organization_id, date_created, date_modified from bidder";
    query_coll!(conn.query(qs, &[]), Bidder)
}
pub fn select_bidders_by_org(conn: &Connection, org_id: &i32) -> Vec<Bidder> {
    let qs = "select id, organization_id, date_created, date_modified \
              from bidder where organization_id = $1";
    query_coll!(conn.query(qs, &[org_id]), Bidder)
}
pub fn insert_bidder(conn: Connection, org_id: i32) -> Result<Bidder, String> {
    let qs = "insert into bidder (organization_id) values ($1) \
              returning id, date_created, date_modified";
    try_insert_to_model!(conn.query(qs, &[&org_id]) ;
                         Bidder ;
                         id: 0, date_created: 1, date_modified: 2 ;
                         organization_id: org_id)
}

// Item related queries
//
pub fn select_items_all(conn: &Connection) -> Vec<Item> {
    let qs = "select id, organization_id, owning_bidder_id, is_goal, title, \
              description, value, min_bid, date_created, date_modified \
              from item";
    query_coll!(conn.query(qs, &[]), Item)
}
pub fn insert_item(conn: Connection, org_id: i32, is_goal: bool,
                   title: String, desc: String, value: i64, min_bid: i64)
                   -> Result<Item, String> {
    let qs = "insert into item (organization_id, is_goal, title, description, value, min_bid) \
              values ($1, $2, $3, $4, $5, $6) \
              returning id, owning_bidder_id, date_created, date_modified";
    try_insert_to_model!(conn.query(qs, &[&org_id, &is_goal, &title, &desc, &value, &min_bid]) ;
                         Item ;
                         id: 0, owning_bidder_id: 1, date_created: 2, date_modified: 3 ;
                         organization_id: org_id, is_goal: is_goal, title: title,
                         description: desc, value: value, min_bid: min_bid)
}

// Profile related queries
//
pub fn select_profiles_all(conn: &Connection) -> Vec<Profile> {
    let qs = "select id, user_id, bidder_id, level_, is_primary, name, \
              phone_cc, phone_number, phone_ext, email, cc_info, extra, \
              date_created, date_modified \
              from profile";
    query_coll!(conn.query(qs, &[]), Profile)
}
pub fn insert_profile(conn: Connection, user_id: i32, bidder_id: Option<i32>, level: i32,
                      is_primary: bool, name: String, phone_cc: Option<String>,
                      phone_number: Option<String>, phone_ext: Option<String>,
                      email: String, cc_info: Option<json::Json>, extra: Option<json::Json>)
                      -> Result<Profile, String> {
    let qs = "insert into profile (user_id, bidder_id, level_, is_primary, name, \
              phone_cc, phone_number, phone_ext, email, cc_info, extra) values \
              ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) \
              returning id, date_created, date_modified";
    try_insert_to_model!(conn.query(qs, &[&user_id, &bidder_id, &level, &is_primary,
                                          &name, &phone_cc, &phone_number, &phone_ext,
                                          &email, &cc_info, &extra]) ;
                         Profile ;
                         id: 0, date_created: 1, date_modified: 2 ;
                         user_id: user_id, bidder_id: bidder_id, level: level,
                         is_primary: is_primary, name: name, phone_cc: phone_cc,
                         phone_number: phone_number, phone_ext: phone_ext,
                         email: email, cc_info: cc_info, extra: extra)
}

// Bid related queries
//
pub fn select_bids_all(conn: &Connection) -> Vec<Bid> {
    let qs = "select id, bidder_id, item_id, amount, date_created, date_modified from bid";
    query_coll!(conn.query(qs, &[]), Bid)
}
pub fn select_bids_by_item(conn: &Connection, item_id: &i32) -> Vec<Bid> {
    let qs = "select id, bidder_id, item_id, amount, date_created, date_modified \
              from bid where item_id = $1";
    query_coll!(conn.query(qs, &[item_id]), Bid)
}
pub fn insert_bid(conn: Connection, bidder_id: i32, item_id: i32, amount: i64) -> Result<Bid, String> {
    let qs = "insert into bid (bidder_id, item_id, amount) values ($1, $2, $3) \
              returning id, date_created, date_modified";
    try_insert_to_model!(conn.query(qs, &[&bidder_id, &item_id, &amount]) ;
                         Bid ;
                         id: 0, date_created: 1, date_modified: 2 ;
                         bidder_id: bidder_id, item_id: item_id, amount: amount)
}
