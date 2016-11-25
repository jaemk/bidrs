
use std::sync::Mutex;

use super::rouille;
use super::r2d2::{Config, Pool};
use super::r2d2_postgres::{PostgresConnectionManager, TlsMode};

use super::sql;
use super::endpoints;
use super::sessions::SessionStore;

use std::io;

pub fn start() {
    let db_url = "postgresql://biddy:biddy@localhost";
    let db_mgr = PostgresConnectionManager::new(db_url, TlsMode::None).expect("connection fail");
    let pool = Pool::new(Config::default(), db_mgr).expect("pool fail");
    println!("connected to db!");

    let session_store = Mutex::new(SessionStore::new());

    rouille::start_server("localhost:3000", move |request| {
        rouille::log(&request, io::stdout(), || {
            if let Some(request) = request.remove_prefix("/resources") {
                let response = rouille::match_assets(&request, "resources");
                if response.success() {
                    return response;
                }
            }
            let conn = pool.clone().get().unwrap();

            let store = session_store.lock().unwrap();
            let sid = match request.header("SID") {
                Some(id) => id,
                None => {
                    if request.url() != "/login" {
                        return rouille::Response::redirect("/login")
                    }
                    "".to_string()
                }
            };
            let session = store.get(&sid);
            println!("session: {:?}", session);

            router!(request,
                (GET) (/) => {
                    println!("redirect");
                    rouille::Response::redirect("/users")
                },

                (GET) (/login) => {
                    // authenticate
                    //let sid = store.add(Session::new()),
                    //let mut resp = rouille::Response::json(&something);
                    //resp.headers.push(("SID".to_string(), sid));
                    rouille::Response::text("please log in")
                },
                (GET) (/users) => {
                    println!("users");
                    let users = sql::select_users_all(&conn);
                    let resp = rouille::Response::json(&users);
                    resp
                },

                (GET) (/users/latest) => {
                    let latest = sql::select_user_latest(&conn).unwrap();
                    rouille::Response::json(&latest)
                },

                _ => rouille::Response::empty_404()
            )
        })
    });
}
