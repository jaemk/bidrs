use std::io::{self, Read, BufRead, Write};
use rpassword::read_password;
use rustc_serialize::json::Json;
use chrono::NaiveDate;

use service::establish_connection;
use models;
use errors::*;


pub struct Prompter {
    msg: String,
    secure: bool,
    multiline: bool,
}
impl Prompter {
    pub fn new(msg: &str) -> Prompter {
        Prompter { msg: msg.into(), secure: false, multiline: false }
    }
    pub fn secure(self) -> Prompter {
        Prompter { msg: self.msg, secure: true, multiline: self.multiline }
    }
    pub fn multiline(self) -> Prompter {
        Prompter { msg: self.msg, secure: self.secure, multiline: true }
    }
    pub fn confirm(self) -> Result<bool> {
        loop {
            print!("{}", self.msg);
            let _ = io::stdout().flush().chain_err(|| "Error flushing stdout")?;
            let mut resp = String::new();
            let stdin = io::stdin();
            let _ = stdin.lock().read_line(&mut resp)
                .chain_err(|| "Error reading stdin")?;
            let clean = resp.trim().to_lowercase();
            if clean == "y" || clean == "n" {
                return Ok(clean == "y");
            }
        }
    }
    pub fn loop_confirm_capture(self) -> Result<String> {
        loop {
            let input = self.capture().chain_err(|| "prompter Error")?;
            println!("\nFound: {:?}", input);
            let ok = Prompter::new("$ ok? (y/n)>> ")
                             .confirm()
                             .chain_err(|| "Prompter Error")?;
            if ok {
                return Ok(input);
            }
        }
    }
    pub fn capture(&self) -> Result<String> {
        print!("{}", self.msg);
        let _ = io::stdout().flush().chain_err(|| "Error flushing stdout")?;
        if self.secure {
            Ok(read_password().chain_err(|| "Error reading secure password")?)
        } else if self.multiline {
            let mut resp = String::new();
            let stdin = io::stdin();
            let _ = stdin.lock().read_to_string(&mut resp)
                .chain_err(|| "Error reading stdin")?;
            Ok(resp.trim().into())
        } else {
            let stdin = io::stdin();
            let mut resp = String::new();
            let _ = stdin.lock().read_line(&mut resp)
                .chain_err(|| "Error reading stdin")?;
            Ok(resp.trim().into())
        }
    }
}


pub fn create_user() -> Result<models::User> {
    println!("Creating new user...");
    let conn = establish_connection();
    let new_email = Prompter::new("$ User email >> ")
                            .capture()
                            .chain_err(|| "Prompter Error")?;
    let new_password = Prompter::new("$ User password >> ")
                               .secure()
                               .capture()
                               .chain_err(|| "Prompter Error")?;
    let new_level = Prompter::new("$ User level >> ").capture()
                             .chain_err(|| "prompter error")?
                             .parse::<i32>()
                             .chain_err(|| "error parsing user.level")?;
    let new_auth: models::Auth = models::NewAuth::new(&new_password).create(&conn)
        .chain_err(|| "Error creating auth")?;
    let new_user: models::User = models::NewUser::new(&new_email, &new_auth, new_level)
        .create(&conn).chain_err(|| "Error creating user")?;
    println!("User created with id, email: {}, {}", new_user.id, new_user.email);
    Ok(new_user)
}


pub fn create_org() -> Result<models::Organization> {
    println!("Creating new organization...");
    let conn = establish_connection();
    let new_name = Prompter::new("$ Org name >> ")
                           .capture()
                           .chain_err(|| "Prompter Error")?;
    let extra = Prompter::new("$ Org details (json, press CTRL+D when finished) >> ")
                        .multiline()
                        .loop_confirm_capture()
                        .chain_err(|| "Prompter Error")?;
    let new_extra = if extra.is_empty() { None } else {
        Some(Json::from_str(&extra).chain_err(|| "Failed to parse OrgExtra json")?)
    };
    let new_org = models::NewOrg::new(&new_name, &new_extra)
        .create(&conn).chain_err(|| "Error creating organization")?;
    println!("Org created with id, name: {}, {}", new_org.id, new_org.name);
    Ok(new_org)
}


pub fn create_bidder() -> Result<models::Bidder> {
    println!("Creating new bidder...");
    let conn = establish_connection();
    let org_id = Prompter::new("$ Org id >> ")
                         .capture()
                         .chain_err(|| "Prompter Error")?
                         .parse::<i32>()
                         .chain_err(|| "Error parsing i32")?;
    let id_name = Prompter::new("$ Bidder id_name >> ")
                          .capture()
                          .chain_err(|| "Prompter Error")?;
    let new_bidder = models::NewBidder::new(org_id, &id_name)
        .create(&conn).chain_err(|| "Error creating bidder")?;
    println!("Bidder created with id, id_name, org-id: {}, {}, {}",
             new_bidder.id, new_bidder.id_name, new_bidder.organization_id);
    Ok(new_bidder)
}


pub fn create_payment_info() -> Result<models::PaymentInfo> {
    println!("Creating new payment information...");
    let conn = establish_connection();
    let args = ["card number", "security pin",
                "expiration date (mm/yyyy)"]
        .iter().map(|arg| {
            Prompter::new(&format!("$ {} >> ", arg))
                    .capture()
                    .expect("Prompter Error")
        }).collect::<Vec<_>>();
    let new_info = models::NewPaymentInfo::new(
        &args[0], &args[1],
        &NaiveDate::parse_from_str(&format!("{}/01", &args[2]),
                                   "%m/%Y/%d")
                   .chain_err(|| "Error parsing cc-expiration")?
    ).create(&conn).chain_err(|| "Error creating payment info")?;
    println!("PaymentInfo created with id, exp: {}, {}",
             new_info.id, new_info.cc_exp);
    Ok(new_info)
}


pub fn create_profile() -> Result<models::Profile> {
    println!("Creating new profile...");
    let conn = establish_connection();
    let args = ["user_id", "bidder_id", "payment_info_id",
                "is_primary", "name", "phone"].iter().map(|arg| {
                    Prompter::new(&format!("$ {} >> ", arg))
                            .capture()
                            .expect("Prompter Error")
                }).collect::<Vec<_>>();
    let extra = Prompter::new("$ Profile extra details (json, press CTRL+D when finished) >> ")
                        .multiline()
                        .loop_confirm_capture()
                        .chain_err(|| "Prompter Error")?;
    let new_extra = if extra.is_empty() { None } else {
        Some(Json::from_str(&extra).chain_err(|| "Failed to parse OrgExtra json")?)
    };
    let new_profile = models::NewProfile::new(
        args[0].parse::<i32>().chain_err(|| "user_id error")?,
        if args[1].is_empty() { None } else { Some(args[1].parse::<i32>().chain_err(|| "bidder_id error")?) },
        if args[2].is_empty() { None } else { Some(args[2].parse::<i32>().chain_err(|| "payment_info_id error")?) },
        args[3].parse::<bool>().chain_err(|| "is_primary error")?,
        &args[4],
        if args[5].is_empty() { None } else { Some(&args[5]) },
        new_extra,
    ).create(&conn).chain_err(|| "Error creating profile")?;
    println!("Profile created with id, name: {}, {}",
             new_profile.id, new_profile.name);
    Ok(new_profile)
}


pub fn create_item() -> Result<models::Item> {
    println!("Creating new item...");
    let conn = establish_connection();
    let args = ["organization_id", "is_goal", "title",
                "description", "value [in cents]", "min_bid [in cents]"].iter().map(|arg| {
                    Prompter::new(&format!("$ {} >> ", arg))
                            .capture()
                            .expect("Prompter Error")
                }).collect::<Vec<_>>();
    let new_item = models::NewItem::new(
        args[0].parse::<i32>().chain_err(|| "org_id error")?,
        args[1].parse::<bool>().chain_err(|| "is_goal error")?,
        &args[2], &args[3],
        args[4].parse::<i64>().chain_err(|| "value error")?,
        args[5].parse::<i64>().chain_err(|| "min_bid error")?,
    ).create(&conn).chain_err(|| "Error creating item")?;
    println!("Item created with id, title: {}, {}",
             new_item.id, new_item.title);
    Ok(new_item)
}


pub fn create_bid() -> Result<models::Bid> {
    println!("Creating new bid...");
    let conn = establish_connection();
    let args = ["bidder_id", "item_id", "amount (in cents)"].iter().map(|arg| {
        Prompter::new(&format!("$ {} >> ", arg))
                .capture()
                .expect("Prompter Error")
    }).collect::<Vec<_>>();
    let new_bid = models::NewBid::new(
        args[0].parse::<i32>().chain_err(|| "bidder_id error")?,
        args[1].parse::<i32>().chain_err(|| "item_id error")?,
        args[2].parse::<i64>().chain_err(|| "amount error")?,
    ).create(&conn).chain_err(|| "Error creating bid")?;
    println!("Bid created with id, amount: {}, {}",
             new_bid.id, new_bid.amount);
    Ok(new_bid)
}
