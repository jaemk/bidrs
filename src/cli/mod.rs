//! CLI
//!
use std::io;
use std::io::Write;
use std::collections::BTreeMap;

use super::postgres::{Connection, TlsMode};
use super::rustc_serialize::json::{Json, ToJson};

use super::sql;
use super::auth;

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

fn get_arg_or(args: &Vec<String>, n: usize, or: &str) -> String {
    let or = or.to_string();
    let arg = args.iter().nth(n).unwrap_or(&or);
    arg.to_string()
}

fn prompt(msg: &str) -> String {
    let stdin = io::stdin();
    let mut resp = String::new();
    print!("{}", msg);
    let _ = io::stdout().flush();
    let _ = stdin.read_line(&mut resp).unwrap();
    resp.trim().to_string()
}

pub fn consume(args: Vec<String>) {
    let arg = get_arg_or(&args, 0, "select");

    let conn = Connection::connect("postgresql://bidrs:bidrs@localhost", TlsMode::None).unwrap();
    println!("Connected to db!");

    if arg == "create-admin" {
        println!("\nCreating a new admin!");
        let email = prompt(" $ email >> ");
        let password = prompt(" $ password >> ");
        let admin_level = prompt(" $ admin-level >> ").parse::<i32>().unwrap();
        let name = prompt(" $ name  >> ");
        let primary = true;
        let salt = auth::new_salt().unwrap();
        let pass_secure = auth::hash(password.as_str(), salt.as_slice()).unwrap();
        let new_user = sql::insert_user(&conn, email, salt, pass_secure).unwrap();
        let new_prof = sql::insert_profile(
            &conn, new_user.id, None, admin_level,
            primary, name, None, None,
            None, None, None).unwrap();
        println!("New user `{}:{}` created!", new_user.email, new_prof.name);
    }

    // user actions
    else if arg == "user-select" {
        let id = get_arg_or(&args, 1, "select");
        let idint = id.parse::<i32>().unwrap_or(1);
        let user = sql::select_user_by_id(&conn, &idint);
        println!("{:?}", user);
    } else if arg == "user-select-all" {
        let users = sql::select_users_all(&conn);
        for user in users.iter() {
            println!("{:?}", user);
        }
    } else if arg == "user-latest" {
        let user = sql::select_user_latest(&conn);
        println!("{:?}", user);
    } else if arg == "user-insert" {
        let email = get_arg_or(&args, 1, "james");
        let pass_raw = get_arg_or(&args, 2, "password");
        let salt = auth::new_salt().unwrap();
        println!("{:?}, len: {:?}", salt, salt.len());
        let pass_secure = auth::hash(pass_raw.as_str(), salt.as_slice()).unwrap();
        let ins = sql::insert_user(&conn, email, salt, pass_secure);
        println!("{:?}", ins);
    }

    // org actions
    else if arg == "org-select" {
        let id = get_arg_or(&args, 1, "");
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
        let name = get_arg_or(&args, 1, "james's org");
        let extra = Extra {
            phone: "5551239876".to_string(),
            address: "123 nut drive".to_string(),
        };
        let ins = sql::insert_org(&conn, name, Some(extra.to_json()));
        println!("{:?}", ins);
    }

    // bidder actions
    else if arg == "bidder-select" {
        let idint = get_arg_or(&args, 1, "").parse::<i32>().unwrap_or(1);
        let bidder = sql::select_bidder_by_id(&conn, &idint);
        println!("{:?}", bidder);
    } else if arg == "bidder-select-all" {
        let bidders = sql::select_bidders_all(&conn);
        for b in bidders.iter() {
            println!("{:?}", b);
        }
    } else if arg == "bidder-where-org" {
        let idint = get_arg_or(&args, 1, "").parse::<i32>().unwrap_or(1);
        let bidders = sql::select_bidders_by_org(&conn, &idint);
        for b in bidders.iter() {
            println!("{:?}", b);
        }
    } else if arg == "bidder-latest" {
        println!("{:?}", sql::select_bidder_latest(&conn));
    } else if arg == "bidder-insert" {
        let org_id = get_arg_or(&args, 1, "").parse::<i32>().unwrap_or(1);
        let ins = sql::insert_bidder(&conn, org_id);
        println!("{:?}", ins);
    }

    // item actions
    else if arg == "item-select-all" {
        let items = sql::select_items_all(&conn);
        for item in items.iter() {
            println!("{:?}", item);
        }
    } else if arg == "item-insert" {
        let org_id = get_arg_or(&args, 1, "").parse::<i32>().unwrap_or(1);
        let ins = sql::insert_item(&conn, org_id, false, "item1".to_string(),
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
        let user_id = get_arg_or(&args, 1, "").parse::<i32>().unwrap_or(1);
        let bidder_id = get_arg_or(&args, 2, "").parse::<i32>().unwrap_or(1);
        let extra = Extra {
            phone: "5551239876".to_string(),
            address: "123 nut drive".to_string(),
        };
        let ins = sql::insert_profile(&conn, user_id, Some(bidder_id), 1, false,
                                      "james".to_string(), None, None, None,
                                      None, Some(extra.to_json()));
        println!("{:?}", ins);
    }

    // bid actions
    else if arg == "bid-select-all" {
        let bids = sql::select_bids_all(&conn);
        for bid in bids.iter() {
            println!("{:?}", bid);
        }
    } else if arg == "bid-insert" {
        let bidder_id = get_arg_or(&args, 1, "").parse::<i32>().unwrap_or(1);
        let item_id = get_arg_or(&args, 2, "").parse::<i32>().unwrap_or(1);

        let ins = sql::insert_bid(&conn, bidder_id, item_id, 100_0000);
        println!("{:?}", ins);
    } else if arg == "bid-select-by-item" {
        let item_id = get_arg_or(&args, 1, "").parse::<i32>().unwrap_or(1);
        let bids = sql::select_bids_by_item(&conn, &item_id);
        for bid in bids.iter() {
            println!("{:?}", bid);
        }
    }
}
