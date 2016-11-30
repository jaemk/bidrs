/// Sessions
///
/// Session & SessionStore impls
/// SessionKey for insertion in iron's request.extensions typemap

use std::collections::HashMap;
use super::uuid::Uuid;
use super::chrono;
use super::jwt::{encode, Header};
use super::iron::typemap;


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


/// SessionKey type to be inserted & retrieved from iron's request typemap
pub struct SessionKey;
impl typemap::Key for SessionKey {
    type Value = Session;
}


#[derive(Debug)]
/// User Session token and information
pub struct Session {
    pub token: String,
    user_uuid: Uuid,
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
    store: HashMap<String, Session>,
    life_span: chrono::Duration,
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
        let token = sess.token.to_string();
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

