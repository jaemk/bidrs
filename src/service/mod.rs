
use super::rouille;
use super::r2d2::{Config, Pool};
use super::r2d2_postgres::{PostgresConnectionManager, TlsMode};

use super::sql;

use std::io;

pub fn start() {
    let db_url = "postgresql://biddy:biddy@localhost";
    let db_mgr = PostgresConnectionManager::new(db_url, TlsMode::None).expect("connection fail");
    let pool = Pool::new(Config::default(), db_mgr).expect("pool fail");

    rouille::start_server("localhost:8000", move |request| {
        rouille::log(&request, io::stdout(), || {
            if let Some(request) = request.remove_prefix("/resources") {
                let response = rouille::match_assets(&request, "resources");
                if response.success() {
                    return response;
                }
            }
            let conn = pool.clone().get().unwrap();
            router!(request,
                (GET) (/) => {
                    println!("redirect");
                    rouille::Response::redirect("/users")
                },

                (GET) (/users) => {
                    println!("users");
                    let users = sql::select_users_all(&conn);
                    rouille::Response::json(&users)
                },

                (GET) (/user/latest) => {
                    let latest = sql::select_user_latest(&conn).unwrap();
                    rouille::Response::json(&latest)
                },

                _ => rouille::Response::empty_404()
            )
        })
    });
}
