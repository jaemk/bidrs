//! Sessions
//!
//! Session & SessionStore impls
//! SessionKey for insertion in iron's request.extensions typemap
//!
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::thread;
use std::time;

use uuid::Uuid;
use chrono;
use jwt::{encode, Header};
use iron::Request;
use iron::headers::Authorization;
use iron::typemap;


#[derive(RustcEncodable, RustcDecodable)]
/// Token json web token generation
struct Claim {
    user_uuid: String,
}
lazy_static! {
    // generate a new session-secret each time the server starts up
    // since the session-store gets wiped out anyway
    static ref SESSION_SECRET: String = Uuid::new_v4().to_string();
}
/// generate a new jwt token for auth/session tokens
fn generate_token(uuid: String) -> Result<String, String> {
    let claim = Claim { user_uuid: uuid };
    match encode(Header::default(), &claim, SESSION_SECRET.as_bytes()) {
        Ok(t) => Ok(t),
        Err(_) => Err("failed to generate a token".to_string()),
    }
}


/// SessionKey type to be inserted & retrieved from iron's request typemap.
/// This currently isn't used since all sessions must be authenticated
/// and the session-token can be found in the request.headers::<Authorization>,
/// but is available if needed.
pub struct SessionKey;
impl typemap::Key for SessionKey {
    type Value = Session;
}


#[derive(Debug)]
/// User Session token and information
pub struct Session {
    pub token: String,
    pub user_uuid: Uuid,
    data: String,
    pub stamp: chrono::DateTime<chrono::UTC>,
}
impl Session {
    pub fn new(uuid: &Uuid) -> Session {
        let token = generate_token(uuid.to_string()).expect("token fail");
        Session {
            token: token,
            user_uuid: uuid.clone(),
            data: "".to_string(),
            stamp: chrono::UTC::now(),
        }
    }
    pub fn expired(&self, life: chrono::Duration) -> bool {
        chrono::UTC::now() - self.stamp > life
    }
    /// update the current session's timestamp
    pub fn touch(&mut self) {
        self.stamp = chrono::UTC::now();
    }
}


#[derive(Debug)]
/// User session store & manager
pub struct SessionStore {
    pub store: HashMap<String, Session>,
    pub life_span: chrono::Duration,
}
impl SessionStore {
    /// Start a new session-store with a specified life-span
    pub fn new(life: i64) -> SessionStore {
        SessionStore {
            store: HashMap::new(),
            life_span: chrono::Duration::seconds(life),
        }
    }

    /// Add a session to the session-store, return its token
    pub fn add(&mut self, sess: Session) -> String {
        let token = sess.token.clone();
        self.store.insert(token.clone(), sess);
        token
    }

    /// Return the number of session entries
    pub fn len(&self) -> usize {
        self.store.len()
    }

    /// Returns true if the session's life-span is within
    /// the life_span specified for the session-store
    pub fn token_expired(&self, token: &String) -> bool {
        match self.store.get(token) {
            Some(sess) => sess.expired(self.life_span),
            None => false,
        }
    }

    /// Check the session-store for a given token. If it exists,
    /// check its timestamp against the store's specified life_span
    /// and delete it if it's expired, otherwise touch it to update
    /// the sessions timestamp.
    pub fn check_delete(&mut self, token: &String) -> bool {
        if self.token_expired(token) {
            self.store.remove(token);
            false
        } else {
            match self.store.get_mut(token) {
                Some(mut sess) => {
                    sess.touch();
                    true
                },
                _ => false,
            }
        }
    }

    /// Returns a mutable reference to the session corresponding to the token.
    pub fn get_mut(&mut self, token: &String) -> Option<&mut Session> {
        self.store.get_mut(token)
    }

    /// Return a mutable reference to the session linked to the request
    /// Authorization token
    pub fn get_mut_from_request(&mut self, request: &Request) -> Option<&mut Session> {
        match request.headers.get::<Authorization<String>>() {
            Some(token) => self.get_mut(token),
            None => None,
        }
    }

    /// Delete from the SessionStore the Session associated with the given
    /// Request's Auth-token
    pub fn delete_by_request(&mut self, request: &Request) -> Option<Session> {
        match request.headers.get::<Authorization<String>>() {
            Some(&Authorization(ref token)) => self.store.remove(token),
            _ => None,
        }
    }

    /// Touch the given session (by token) to update its timestamp
    /// Returns an Error if the token does not exist in the store.
    pub fn touch(&mut self, token: &String) -> Result<(), ()> {
        match self.store.get_mut(token) {
            Some(mut sess) => {
                sess.touch();
                Ok(())
            }
            None => Err(()),
        }
    }
}


/// Start a daemon thread to clean out stale sessions left in
/// the session-store every 'interval' seconds.
pub fn start_daemon_sweeper(session_store: Arc<Mutex<SessionStore>>, interval: u64) {
    // startup session daemon
    thread::spawn(move || {
        loop {
            {
                let mut s_store = session_store.lock().unwrap();
                let stale = s_store.store.iter().fold(vec![], |mut acc, (k, v)| {
                    let now_ish = chrono::UTC::now();
                    if now_ish - v.stamp > s_store.life_span {
                        acc.push(k.clone());
                    }
                    acc
                });
                let mut count = 0;
                for k in stale.iter() {
                    s_store.store.remove(k);
                    count += 1;
                }
                println!(">> Cleaned out {} stale sessions", count);
            }
            thread::sleep(time::Duration::from_secs(interval));
        }
    });
}
