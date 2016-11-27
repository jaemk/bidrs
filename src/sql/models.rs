use super::super::chrono;
use super::super::uuid;
use super::super::postgres;
use super::super::rustc_serialize::json::{Json}; //, ToJson};

// TODO: impl ToJson to_json for any chrono containing structs.
//       export them as strings in their debug format.

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub uuid: uuid::Uuid,
    pub date_created: chrono::DateTime<chrono::UTC>,
    pub date_modified: chrono::DateTime<chrono::UTC>,
}
impl User {
    pub fn from_row(row: postgres::rows::Row) -> User {
        User {
            id: row.get(0),
            username: row.get(1),
            uuid: row.get(2),
            date_created: row.get(3),
            date_modified: row.get(4),
        }
    }
}

#[derive(Debug, RustcEncodable)]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub extra: Option<Json>,
    pub date_created: chrono::DateTime<chrono::UTC>,
    pub date_modified: chrono::DateTime<chrono::UTC>,
}
impl Organization {
    pub fn from_row(row: postgres::rows::Row) -> Organization {
        Organization {
            id: row.get(0),
            name: row.get(1),
            extra: row.get(2),
            date_created: row.get(3),
            date_modified: row.get(4),
        }
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Bidder {
    pub id: i32,
    pub organization_id: i32,   // *
    pub date_created: chrono::DateTime<chrono::UTC>,
    pub date_modified: chrono::DateTime<chrono::UTC>,
}
impl Bidder {
    pub fn from_row(row: postgres::rows::Row) -> Bidder {
        Bidder {
            id: row.get(0),
            organization_id: row.get(1),
            date_created: row.get(2),
            date_modified: row.get(3),
        }
    }
}

#[derive(Debug, RustcEncodable)]
pub struct Item {
    pub id: i32,
    pub organization_id: i32,   // *
    pub owning_bidder_id: Option<i32>,
    pub is_goal: bool,
    pub title: String,          // *
    pub description: String,    // *
    pub value: i64,             // *
    pub min_bid: i64,           // *
    pub date_created: chrono::DateTime<chrono::UTC>,
    pub date_modified: chrono::DateTime<chrono::UTC>,
}
impl Item {
    pub fn from_row(row: postgres::rows::Row) -> Item {
        Item {
            id: row.get(0),
            organization_id: row.get(1),
            owning_bidder_id: row.get(2),
            is_goal: row.get(3),
            title: row.get(4),
            description: row.get(5),
            value: row.get(6),
            min_bid: row.get(7),
            date_created: row.get(8),
            date_modified: row.get(9),
        }
    }
}

#[derive(Debug, RustcEncodable)]
pub struct Profile {
    pub id: i32,
    pub user_id: i32,
    pub bidder_id: Option<i32>,
    pub level: i32,
    pub is_primary: bool,
    pub name: String,
    pub phone_cc: Option<String>,
    pub phone_number: Option<String>,
    pub phone_ext: Option<String>,
    pub email: String,
    pub cc_info: Option<Json>,
    pub extra: Option<Json>,
    pub date_created: chrono::DateTime<chrono::UTC>,
    pub date_modified: chrono::DateTime<chrono::UTC>,
}
impl Profile {
    pub fn from_row(row: postgres::rows::Row) -> Profile {
        Profile {
            id: row.get(0), user_id: row.get(1),
            bidder_id: row.get(2), level: row.get(3),
            is_primary: row.get(4), name: row.get(5),
            phone_cc: row.get(6), phone_number: row.get(7),
            phone_ext: row.get(8), email: row.get(9),
            cc_info: row.get(10), extra: row.get(11),
            date_created: row.get(12), date_modified: row.get(13),
        }
    }
}

#[derive(Debug, RustcEncodable, RustcDecodable)]
pub struct Bid {
    pub id: i32,
    pub bidder_id: i32,
    pub item_id: i32,
    pub amount: i64,
    pub date_created: chrono::DateTime<chrono::UTC>,
    pub date_modified: chrono::DateTime<chrono::UTC>,
}
impl Bid {
    pub fn from_row(row: postgres::rows::Row) -> Bid {
        Bid {
            id: row.get(0), bidder_id: row.get(1),
            item_id: row.get(2), amount: row.get(3),
            date_created: row.get(4), date_modified: row.get(5),
        }
    }
}

