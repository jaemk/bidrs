
use std::collections::HashMap;
use super::uuid;

#[derive(Debug)]
pub struct Session {
    pub id: uuid::Uuid,
    //user_uuid: uuid::Uuid,
}

impl Session {
    pub fn new() -> Session {
        // TODO: use a proper token
        Session {
            id: uuid::Uuid::new_v4(),
        }
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
    pub fn get(&self, sid: &String) -> Option<&Session> {
        let &SessionStore(ref store) = self;
        store.get(sid)
    }
}

