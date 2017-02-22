extern crate bidrs;
extern crate uuid;

use uuid::Uuid;
use bidrs::models;
use bidrs::service;
use bidrs::auth;


pub fn main() {
    let conn = service::establish_connection();
    conn.execute("BEGIN", &[]).unwrap();
    println!("Begin transaction\n");

    // add users
    let user_emails = ["james.k@gmail.com", "bob@gmail.com", "lauren@gmail.com", "brian.k@gmail.com"];
    let mut users = vec![];
    for email in user_emails.iter() {
        let salt = auth::new_salt().expect("failed salt");
        let new_auth = models::NewAuth { salt: salt, password: "enter".into() };
        let auth = new_auth.create(&conn).expect("failed auth creation");

        let new_user = models::NewUser { auth_id: auth.id, level_: 10, email: email.to_string(), uuid_: Uuid::new_v4() };
        let user = new_user.create(&conn).expect("failed user creation");

        println!("created user [{}] with id={}", user.email, user.id);
        users.push(user);
    }

    // add org
    let new_org = models::NewOrg { name: "Cool Orgnaization".into(), extra: None };
    let org = new_org.create(&conn).expect("failed org creation");
    println!("\ncreated org [{}] with id={}\n", org.name, org.id);

    let mut profiles = vec![];
    for (user, name) in users.iter().zip(["james", "bob", "lauren", "brian"].iter()) {
        let new_prof = models::NewProfile {
            user_id: user.id, bidder_id: None, payment_info_id: None,
            is_primary: false, name: name.to_string(),
            phone: None, extra: None,
        };
        let profile = new_prof.create(&conn)
            .expect(&format!("failed to create profile for: {}", name));

        println!("created profile for [{}] with id={}", profile.name, profile.id);
        profiles.push(profile);
    }

    // add bidders
    println!("");
    let bidder_names = [vec!["james", "lauren"], vec!["brian"], vec!["bob"]];
    let mut bidders = vec![];
    for names in bidder_names.into_iter() {
        let id_name = names.join("_").to_lowercase();
        let new_bidder = models::NewBidder { organization_id: org.id, id_name: id_name };
        let bidder = new_bidder.create(&conn).expect("failed bidder creation");

        println!("created bidder [{}] with id={}", bidder.id_name, bidder.id);

        let mut first = true;
        for name in names.iter() {
            let mut qs = "update profiles set bidder_id=$1 where name=$2";
            if first {
                qs = "update profiles set is_primary='true', bidder_id=$1 where name=$2";
                println!("setting {} as primary bidder", name);
                first = false;
            }
            println!("setting {}'s bidder_id={}", name, bidder.id);
            conn.execute(qs, &[&Some(bidder.id), &name]).expect("failed updating user");
        }
        bidders.push(bidder);
    }

    // add items
    println!("");
    let item_info = [("box", 10000, 5000, 1000), ("sphere", 20000, 7500, 2000), ("paper", 15000, 2000, 500)];
    let mut items = vec![];
    for info in item_info.into_iter() {
        let new_item = models::NewItem {
            organization_id: org.id, owning_bidder_id: None, is_goal: false,
            title: info.0.to_string(), description: info.0.to_string(),
            value: info.1, starting: info.2, min_bid: info.3,
        };
        let item = new_item.create(&conn).expect("failed creating item");

        println!("created item [{}] with id={}", item.title, item.id);
        items.push(item);
    }

    // add bid
    println!("");
    let new_bid = models::NewBid { bidder_id: bidders[0].id, item_id: items[0].id, amount: 5000 };
    let bid = new_bid.create(&conn).expect("failed creating bid");
    println!("created bid [{}] on item={}", bid.amount, bid.item_id);

    conn.execute("COMMIT", &[]).expect("failed transaction");
    println!("End transaction, success");
}
