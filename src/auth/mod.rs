
use super::uuid::Uuid;

pub fn new_salt() -> String {
    Uuid::new_v4().to_string()
}

pub fn hash() -> String {
    unimplemented!()
}
