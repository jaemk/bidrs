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
    // generate a new secret each time the server starts up
    static ref SECRET: String = Uuid::new_v4().to_string();
}
/// generate a new jwt token for auth/session tokens
fn generate_token(uuid: String) -> Result<String, String> {
    let claim = Claim { user_uuid: uuid };
    match encode(Header::default(), &claim, SECRET.as_bytes()) {
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
    //user_uuid: Uuid,
    //data: json::Json,
    pub stamp: chrono::DateTime<chrono::UTC>,
}
impl Session {
    pub fn new(uuid: &Uuid) -> Session {
        let token = generate_token(uuid.to_string()).expect("token fail");
        Session {
            token: token,
            stamp: chrono::UTC::now(),
        }
    }
    /// update the current session's timestamp
    pub fn touch(&mut self) {
        self.stamp = chrono::UTC::now();
    }
}


/// User session store & manager
pub struct SessionStore(HashMap<String, Session>);
impl SessionStore {
    pub fn new() -> SessionStore {
        SessionStore(HashMap::new())
    }
    pub fn add(&mut self, sess: Session) -> String {
        let token = sess.token.to_string();
        let &mut SessionStore(ref mut store) = self;
        store.insert(token.clone(), sess);
        token
    }
    pub fn get_mut(&mut self, token: &String) -> Option<&mut Session> {
        let &mut SessionStore(ref mut store) = self;
        match store.get_mut(token) {
            Some(mut sess) => {
                sess.touch();
                Some(sess)
            }
            None => None,
        }
    }
    pub fn touch(&mut self, token: &String) -> Result<(), ()> {
        let &mut SessionStore(ref mut store) = self;
        match store.get_mut(token) {
            Some(mut sess) => {
                sess.touch();
                Ok(())
            }
            None => Err(()),
        }
    }
}

