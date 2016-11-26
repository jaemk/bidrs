use std::collections::HashMap;
use super::uuid;
use super::chrono;

#[derive(Debug)]
pub struct Session {
    pub id: uuid::Uuid,
    //user_uuid: uuid::Uuid,
    //data: json::Json,
    pub stamp: chrono::DateTime<chrono::UTC>,
}

impl Session {
    pub fn new() -> Session {
        // TODO: use a proper token
        Session {
            id: uuid::Uuid::new_v4(),
            stamp: chrono::UTC::now(),
        }
    }
    pub fn touch(&mut self) {
        self.stamp = chrono::UTC::now();
    }
}

pub struct SessionStore(HashMap<String, Session>);

impl SessionStore {
    pub fn new() -> SessionStore {
        SessionStore(HashMap::new())
    }
    pub fn add(&mut self, sess: Session) -> String {
        let id = sess.id.to_string();
        let &mut SessionStore(ref mut store) = self;
        store.insert(id.clone(), sess);
        id
    }
    pub fn get(&mut self, sid: &String) -> Option<&mut Session> {
        let &mut SessionStore(ref mut store) = self;
        match store.get_mut(sid) {
            Some(mut sess) => {
                sess.touch();
                Some(sess)
            }
            None => None,
        }
    }
    pub fn touch(&mut self, sid: &String) -> Result<(), ()> {
        let &mut SessionStore(ref mut store) = self;
        match store.get_mut(sid) {
            Some(mut sess) => {
                sess.touch();
                Ok(())
            }
            None => Err(()),
        }
    }
}

