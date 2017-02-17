//! Models
//!
//!
use chrono;
use uuid;
use postgres::{self, Connection};
use rustc_serialize::json::{Json};

use auth as app_auth;
use errors::*;


#[derive(Debug, RustcEncodable)]
pub struct Auth {
    pub id: i32,
    pub salt: Vec<u8>,
    pub password: Vec<u8>,
    pub date_created: chrono::DateTime<chrono::UTC>,
    pub date_modified: chrono::DateTime<chrono::UTC>,
}
impl Auth {
    pub fn get(conn: &Connection, id: &i32) -> Option<Auth> {
        let qs = "select * from auth where id=$1";
        query_or_none!(conn.query(qs, &[&id]), Auth)
    }

    fn from_row(row: postgres::rows::Row) -> Auth {
        Auth {
            id: row.get(0),
            salt: row.get(1),
            password: row.get(2),
            date_created: row.get(3),
            date_modified: row.get(4),
        }
    }
}

#[derive(Debug)]
pub struct NewAuth {
    pub salt: Vec<u8>,
    pub password: Vec<u8>,
}
impl NewAuth {
    pub fn new(password_string: &str) -> NewAuth {
        let salt = app_auth::new_salt().expect("salt generation fail");
        let password = app_auth::hash(password_string, &salt).expect("hash fail");
        NewAuth {
            salt: salt,
            password: password,
        }
    }
    pub fn create(self, conn: &Connection) -> Result<Auth> {
        let qs = "insert into auth (salt, password) values ($1, $2) \
                  returning id, date_created, date_modified";
        try_insert_to_model!(conn.query(qs, &[&self.salt, &self.password]) ;
                             Auth ;
                             id: 0, date_created: 1, date_modified: 2 ;
                             salt: self.salt, password: self.password)
    }
}




#[derive(Debug, RustcEncodable)]
pub struct User {
    pub id: i32,
    pub auth_id: i32,
    pub email: String,
    pub uuid_: uuid::Uuid,
    pub date_created: chrono::DateTime<chrono::UTC>,
    pub date_modified: chrono::DateTime<chrono::UTC>,
}
impl User {
    pub fn get(conn: &Connection, id: &i32) -> Option<User> {
        let qs = "select * from users where id=$1";
        query_or_none!(conn.query(qs, &[&id]), User)
    }

    pub fn from_row(row: postgres::rows::Row) -> User {
        User {
            id: row.get(0),
            auth_id: row.get(1),
            email: row.get(2),
            uuid_: row.get(3),
            date_created: row.get(4),
            date_modified: row.get(5),
        }
    }
}

#[derive(Debug)]
pub struct NewUser {
    pub auth_id: i32,
    pub email: String,
    pub uuid_: uuid::Uuid,
}
impl NewUser {
    pub fn new(email: &str, auth: &Auth) -> NewUser {
        let uuid_ = uuid::Uuid::new_v4();
        NewUser {
            auth_id: auth.id,
            email: email.into(),
            uuid_: uuid_,
        }
    }
    pub fn create(self, conn: &Connection) -> Result<User> {
        let qs = "insert into users (auth_id, email, uuid_) values ($1, $2, $3) \
                  returning id, date_created, date_modified";
        try_insert_to_model!(conn.query(qs, &[&self.auth_id, &self.email, &self.uuid_]) ;
                             User ;
                             id: 0, date_created: 1, date_modified: 2 ;
                             auth_id: self.auth_id, email: self.email, uuid_: self.uuid_)
    }
}




//#[derive(Debug, RustcEncodable, RustcDecodable)]
//pub struct OrgExtra {
//    address: String,
//}
//impl ToJson for OrgExtra {
//    fn to_json(&self) -> Json {
//        use ::std::collections::BTreeMap;
//        let mut obj = BTreeMap::new();
//        obj.insert("address".to_string(), self.address.to_json());
//        Json::Object(obj)
//    }
//}

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

#[derive(Debug)]
pub struct NewOrg {
    pub name: String,
    pub extra: Option<Json>,
}
impl NewOrg {
    pub fn new(name: &str, extra: &Option<Json>) -> NewOrg {
        NewOrg {
            name: name.into(),
            extra: extra.clone(),
            //extra: extra.map(|extra: &OrgExtra| extra.to_json()),
        }
    }
    pub fn create(self, conn: &Connection) -> Result<Organization> {
        let qs = "insert into organizations (name, extra) values ($1, $2) \
                  returning id, date_created, date_modified";
        try_insert_to_model!(conn.query(qs, &[&self.name, &self.extra]) ;
                             Organization ;
                             id: 0, date_created: 1, date_modified: 2 ;
                             name: self.name, extra: self.extra)
    }
}




#[derive(Debug, RustcEncodable)]
pub struct Bidder {
    pub id: i32,
    pub organization_id: i32,
    pub id_name: String,
    pub date_created: chrono::DateTime<chrono::UTC>,
    pub date_modified: chrono::DateTime<chrono::UTC>,
}
impl Bidder {
    pub fn from_row(row: postgres::rows::Row) -> Bidder {
        Bidder {
            id: row.get(0),
            organization_id: row.get(1),
            id_name: row.get(2),
            date_created: row.get(2),
            date_modified: row.get(3),
        }
    }
}

#[derive(Debug)]
pub struct NewBidder {
    pub organization_id: i32,
    pub id_name: String,
}
impl NewBidder {
    pub fn new(org_id: i32, id_name: &str) -> NewBidder {
        NewBidder {
            organization_id: org_id,
            id_name: id_name.into(),
        }
    }
    pub fn create(self, conn: &Connection) -> Result<Bidder> {
        let qs = "insert into bidders (organization_id, id_name) values ($1, $2) \
                  returning id, date_created, date_modified";
        try_insert_to_model!(conn.query(qs, &[&self.organization_id, &self.id_name]) ;
                             Bidder ;
                             id: 0, date_created: 1, date_modified: 2 ;
                             organization_id: self.organization_id, id_name: self.id_name)
    }
}




#[derive(Debug, RustcEncodable)]
pub struct PaymentInfo {
    pub id: i32,
    pub cc_number: String,
    pub cc_pin: String,
    pub cc_exp: chrono::NaiveDate,
    pub date_created: chrono::DateTime<chrono::UTC>,
    pub date_modified: chrono::DateTime<chrono::UTC>,
}
impl PaymentInfo {
    pub fn from_row(row: postgres::rows::Row) -> PaymentInfo {
        PaymentInfo {
            id: row.get(0),
            cc_number: row.get(1),
            cc_pin: row.get(2),
            cc_exp: row.get(3),
            date_created: row.get(4),
            date_modified: row.get(5),
        }
    }
}

#[derive(Debug)]
pub struct NewPaymentInfo {
    cc_number: String,
    cc_pin: String,
    cc_exp: chrono::NaiveDate,
}
impl NewPaymentInfo {
    pub fn new(cc_num: &str, cc_pin: &str, cc_exp: &chrono::NaiveDate) -> NewPaymentInfo {
        NewPaymentInfo {
            cc_number: cc_num.into(),
            cc_pin: cc_pin.into(),
            cc_exp: cc_exp.clone(),
        }
    }
    pub fn create(self, conn: &Connection) -> Result<PaymentInfo> {
        let qs = "insert into payment_information (cc_number, cc_pin, cc_exp) values ($1, $2, $3) \
                  returning id, date_created, date_modified";
        try_insert_to_model!(conn.query(qs, &[&self.cc_number, &self.cc_pin, &self.cc_exp]) ;
                             PaymentInfo ;
                             id: 0, date_created: 1, date_modified: 2;
                             cc_number: self.cc_number, cc_pin: self.cc_pin, cc_exp: self.cc_exp)
    }
}




#[derive(Debug, RustcEncodable)]
pub struct Profile {
    pub id: i32,
    pub user_id: i32,
    pub bidder_id: Option<i32>,
    pub payment_info_id: Option<i32>,
    pub level_: i32,
    pub is_primary: bool,
    pub name: String,
    pub phone: Option<String>,
    pub extra: Option<Json>,
    pub date_created: chrono::DateTime<chrono::UTC>,
    pub date_modified: chrono::DateTime<chrono::UTC>,
}
impl Profile {
    pub fn from_row(row: postgres::rows::Row) -> Profile {
        Profile {
            id: row.get(0),
            user_id: row.get(1),
            bidder_id: row.get(2),
            payment_info_id: row.get(3),
            level_: row.get(4),
            is_primary: row.get(5),
            name: row.get(6),
            phone: row.get(7),
            extra: row.get(8),
            date_created: row.get(9),
            date_modified: row.get(10),
        }
    }
}

#[derive(Debug)]
pub struct NewProfile {
    pub user_id: i32,
    pub bidder_id: Option<i32>,
    pub payment_info_id: Option<i32>,
    pub level_: i32,
    pub is_primary: bool,
    pub name: String,
    pub phone: Option<String>,
    pub extra: Option<Json>,
}
impl NewProfile {
    pub fn new(user_id: i32, bidder_id: Option<i32>, payment_info_id: Option<i32>,
               level_: i32, is_primary: bool, name: &str, phone: Option<&str>,
               extra: Option<Json>) -> NewProfile {
        NewProfile {
            user_id: user_id, bidder_id: bidder_id, payment_info_id: payment_info_id,
            level_: level_, is_primary: is_primary, name: name.into(), phone: phone.map(|p| p.into()),
            extra: extra,
        }

    }
    pub fn create(self, conn: &Connection) -> Result<Profile> {
        let qs = "insert into profiles (user_id, bidder_id, payment_info_id, level_, \
                  is_primary, name, phone, extra) values ($1, $2, $3, $4, $5, $6, $7, $8) \
                  returning id, date_created, date_modified";
        try_insert_to_model!(conn.query(qs, &[&self.user_id, &self.bidder_id, &self.payment_info_id,
                                              &self.level_, &self.is_primary, &self.name,
                                              &self.phone, &self.extra]) ;
                             Profile ;
                             id: 0, date_created: 1, date_modified: 2 ;
                             user_id: self.user_id, bidder_id: self.bidder_id,
                             payment_info_id: self.payment_info_id, level_: self.level_,
                             is_primary: self.is_primary, name: self.name,
                             phone: self.phone, extra: self.extra)
    }
}




#[derive(Debug, RustcEncodable)]
pub struct Item {
    pub id: i32,
    pub organization_id: i32,
    pub owning_bidder_id: Option<i32>,
    pub is_goal: bool,
    pub title: String,
    pub description: String,
    pub value: i64,
    pub min_bid: i64,
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




#[derive(Debug)]
pub struct NewItem {
    pub organization_id: i32,
    pub owning_bidder_id: Option<i32>,
    pub is_goal: bool,
    pub title: String,
    pub description: String,
    pub value: i64,
    pub min_bid: i64,
}
impl NewItem {
    pub fn new(org_id: i32, is_goal: bool, title: &str, desc: &str,
               value: i64, min_bid: i64) -> NewItem {
        NewItem {
            organization_id: org_id, is_goal: is_goal, title: title.into(),
            description: desc.into(), value: value, min_bid: min_bid,
            owning_bidder_id: None,
        }
    }
    pub fn create(self, conn: &Connection) -> Result<Item> {
        let qs = "insert into items (organization_id, is_goal, title, description, value, min_bid, owning_bidder_id) \
                  values ($1, $2, $3, $4, $5, $6, $7) returning id, date_created, date_modified";
        try_insert_to_model!(conn.query(qs, &[&self.organization_id, &self.is_goal, &self.title,
                                              &self.description, &self.value, &self.min_bid, &self.owning_bidder_id]) ;
                             Item ;
                             id: 0, date_created: 1, date_modified: 2 ;
                             organization_id: self.organization_id, is_goal:self.is_goal, title: self.title,
                             description: self.description, value: self.value, min_bid: self.min_bid,
                             owning_bidder_id: self.owning_bidder_id)
    }
}





#[derive(Debug, RustcEncodable)]
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
            id: row.get(0),
            bidder_id: row.get(1),
            item_id: row.get(2),
            amount: row.get(3),
            date_created: row.get(4),
            date_modified: row.get(5),
        }
    }
}

#[derive(Debug)]
pub struct NewBid {
    pub bidder_id: i32,
    pub item_id: i32,
    pub amount: i64,
}
impl NewBid {
    pub fn new(bidder_id: i32, item_id: i32, amount: i64) -> NewBid {
        NewBid {
            bidder_id: bidder_id, item_id: item_id, amount: amount,
        }
    }
    pub fn create(self, conn: &Connection) -> Result<Bid> {
        let qs = "insert into bids (bidder_id, item_id, amount) values ($1, $2, $3) \
                  returning id, date_created, date_modified";
        try_insert_to_model!(conn.query(qs, &[&self.bidder_id, &self.item_id, &self.amount]) ;
                             Bid ;
                             id: 0, date_created: 1, date_modified: 2 ;
                             bidder_id: self.bidder_id, item_id: self.item_id, amount: self.amount)
    }
}
