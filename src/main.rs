#[macro_use]
extern crate biddy;
extern crate postgres;
extern crate chrono;
extern crate uuid;
extern crate rustc_serialize;

use std::env;
use std::collections::BTreeMap;
use postgres::{Connection, TlsMode};
use rustc_serialize::json::{Json, ToJson};

use biddy::{sql};

#[derive(RustcEncodable, RustcDecodable)]
struct Extra {
    phone: String,
    address: String,
}
impl ToJson for Extra {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();
        map.insert("phone".to_string(), self.phone.to_json());
        map.insert("address".to_string(), self.address.to_json());
        Json::Object(map)
    }
}

pub fn main() {
    let arg = env::args().nth(1).unwrap_or("select".to_string());

    let conn = Connection::connect("postgresql://biddy:biddy@localhost", TlsMode::None).unwrap();
    println!("Connected to db!");

    // user actions
    if arg == "user-select" {
        let id = env::args().nth(2).unwrap_or("1".to_string());
        let idint = id.parse::<i32>().unwrap_or(1);
        let user = biddy::sql::select_user_by_id(&conn, &idint);
        println!("{:?}", user);
    } else if arg == "user-select-all" {
        let users = sql::select_users_all(&conn);
        for user in users.iter() {
            println!("{:?}", user);
        }
    } else if arg == "user-latest" {
        let user = biddy::sql::select_user_latest(&conn);
        println!("{:?}", user);
    } else if arg == "user-insert" {
        let username = env::args().nth(2).unwrap_or("james".to_string());
        let ins = biddy::sql::insert_user(conn, username);
        println!("{:?}", ins);
    }

    // org actions
    else if arg == "org-select" {
        let id = env::args().nth(2).unwrap_or("".to_string());
        let idint = id.parse::<i32>().unwrap_or(1);
        let org = sql::select_org_by_id(&conn, &idint);
        println!("{:?}", org);
        match org {
            Some(org) => match org.extra {
                Some(extra) => println!("{:?}", extra.as_object()),
                _ => println!("no extra org info"),
            },
            _ => (),
        };
    } else if arg == "org-select-all" {
        let orgs = sql::select_orgs_all(&conn);
        for org in orgs.iter() {
            println!("{:?}", org);
        }
    } else if arg == "org-latest" {
        let org = sql::select_org_latest(&conn);
        println!("{:?}", org);
    } else if arg == "org-insert" {
        let name = env::args().nth(2).unwrap_or("james's org".to_string());
        let extra = Extra {
            phone: "5551239876".to_string(),
            address: "123 nut drive".to_string(),
        };
        let ins = sql::insert_org(conn, name, Some(extra.to_json()));
        println!("{:?}", ins);
    }

    // bidder actions
    else if arg == "bidder-select" {
        let idint = env::args().nth(2).unwrap_or("".to_string()).parse::<i32>().unwrap_or(1);
        let bidder = sql::select_bidder_by_id(&conn, &idint);
        println!("{:?}", bidder);
    } else if arg == "bidder-select-all" {
        let bidders = sql::select_bidders_all(&conn);
        for b in bidders.iter() {
            println!("{:?}", b);
        }
    } else if arg == "bidder-where-org" {
        let idint = env::args().nth(2).unwrap_or("".to_string()).parse::<i32>().unwrap_or(1);
        let bidders = sql::select_bidders_by_org(&conn, &idint);
        for b in bidders.iter() {
            println!("{:?}", b);
        }
    } else if arg == "bidder-latest" {
        println!("{:?}", sql::select_bidder_latest(&conn));
    } else if arg == "bidder-insert" {
        let org_id = env::args().nth(2).unwrap_or("".to_string()).parse::<i32>().unwrap_or(1);
        let ins = sql::insert_bidder(conn, org_id);
        println!("{:?}", ins);
    }

    // item actions
    else if arg == "item-select-all" {
        let items = sql::select_items_all(&conn);
        for item in items.iter() {
            println!("{:?}", item);
        }
    } else if arg == "item-insert" {
        let org_id = env::args().nth(2).unwrap_or("".to_string()).parse::<i32>().unwrap_or(1);
        let ins = sql::insert_item(conn, org_id, false, "item1".to_string(),
                                   "item1-desc".to_string(), 2000_0000i64, 100_0000i64);
        println!("{:?}", ins);
    }

    // profile actions
    else if arg == "profile-select-all" {
        let profiles = sql::select_profiles_all(&conn);
        for prof in profiles.iter() {
            println!("{:?}", prof);
        }
    } else if arg == "profile-insert" {
        let user_id = env::args().nth(2).unwrap_or("".to_string()).parse::<i32>().unwrap_or(1);
        let bidder_id = env::args().nth(3).unwrap_or("".to_string()).parse::<i32>().unwrap_or(1);
        let extra = Extra {
            phone: "5551239876".to_string(),
            address: "123 nut drive".to_string(),
        };
        let ins = sql::insert_profile(conn, user_id, Some(bidder_id), 1, false,
                                      "james".to_string(), None, None, None,
                                      "james@gmail.com".to_string(), None,
                                      Some(extra.to_json()));
        println!("{:?}", ins);
    }

    // bid actions
    else if arg == "bid-select-all" {
        let bids = sql::select_bids_all(&conn);
        for bid in bids.iter() {
            println!("{:?}", bid);
        }
    } else if arg == "bid-insert" {
        let bidder_id = env::args().nth(2).unwrap_or("".to_string()).parse::<i32>().unwrap_or(1);
        let item_id = env::args().nth(3).unwrap_or("".to_string()).parse::<i32>().unwrap_or(1);
        let ins = sql::insert_bid(conn, bidder_id, item_id, 100_0000);
        println!("{:?}", ins);
    } else if arg == "bid-select-by-item" {
        let item_id = env::args().nth(2).unwrap_or("".to_string()).parse::<i32>().unwrap_or(1);
        let bids = sql::select_bids_by_item(&conn, &item_id);
        for bid in bids.iter() {
            println!("{:?}", bid);
        }
    }
}
