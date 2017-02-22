extern crate bidrs;
extern crate uuid;

use uuid::Uuid;
use bidrs::models;
use bidrs::sql;
use bidrs::service;
use bidrs::auth;


pub fn main() {
    let conn = service::establish_connection();
    conn.execute("BEGIN", &[]).unwrap();
    println!("Begin transaction\n");

    let user_emails = ["james.k@gmail.com", "bob@gmail.com", "lauren@gmail.com", "brian.k@gmail.com"];
    let mut users = vec![];
    for email in user_emails.iter() {
        let salt = auth::new_salt().unwrap();
        let new_auth = models::NewAuth { salt: salt, password: "enter".into() };
        let auth = new_auth.create(&conn).unwrap();

        let new_user = models::NewUser { auth_id: auth.id, level_: 10, email: email.to_string(), uuid_: Uuid::new_v4() };
        let user = new_user.create(&conn).unwrap();

        println!("created user [{}] with id={}", user.email, user.id);
        users.push(user);
    }

    let new_org = models::NewOrg { name: "Cool Orgnaization".into(), extra: None };
    let org = new_org.create(&conn).unwrap();
    println!("\ncreated org [{}] with id={}\n", org.name, org.id);

    let mut profiles = vec![];
    for (user, name) in users.iter().zip(["james", "bob", "lauren", "brian"].iter()) {
        let new_prof = models::NewProfile {
            user_id: user.id, bidder_id: None, payment_info_id: None,
            is_primary: false, name: name.to_string(),
            phone: None, extra: None,
        };
        let profile = new_prof.create(&conn).expect(&format!("failed to create profile for: {}", name));

        println!("created profile for [{}] with id={}", profile.name, profile.id);
        profiles.push(profile);
    }

    println!("");
    let bidder_names = [vec!["james", "lauren"], vec!["brian"], vec!["bob"]];
    let mut bidders = vec![];
    for names in bidder_names.into_iter() {
        let id_name = names.join("_").to_lowercase();
        let new_bidder = models::NewBidder { organization_id: org.id, id_name: id_name };
        let bidder = new_bidder.create(&conn).unwrap();

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
            conn.execute(qs, &[&Some(bidder.id), &name]).unwrap();
        }
        bidders.push(bidder);
    }


    conn.execute("COMMIT", &[]).unwrap();
    println!("End transaction, success");
}
