extern crate chrono;
extern crate uuid;
extern crate rustc_serialize;
extern crate postgres;


#[derive(Debug)]
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

#[derive(Debug)]
pub struct Organization {
    pub id: i32,
    pub name: String,
    pub extra: rustc_serialize::json::Json,
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

#[derive(Debug)]
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

#[derive(Debug)]
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
