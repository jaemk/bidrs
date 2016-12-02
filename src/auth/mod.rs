//! Auth
//!
//! Hash and Salt generation
//!
use super::rand::{Rng, StdRng};
use super::crypto::bcrypt;


pub fn new_salt() -> Result<Vec<u8>, String> {
    const SALT_SIZE: usize = 16;
    let mut salt = [0u8; SALT_SIZE];
    match StdRng::new() {
        Ok(mut rng) => {
            rng.fill_bytes(&mut salt);
        },
        Err(_) => return Err("rng error".to_string()),
    };
    Ok(salt.iter().cloned().collect::<Vec<u8>>())
}

pub fn hash(string: &str, salt: &[u8]) -> Result<Vec<u8>, String> {
    if salt.len() != 16 || (string.len() == 0 || string.len() > 72) {
        return Err("Salt or String error".to_string());
    }
    const COST: u32 = 10;
    const OUTPUT_SIZE: usize = 24;
    let mut hashed = [0u8; OUTPUT_SIZE];
    bcrypt::bcrypt(COST, salt, string.as_bytes(), &mut hashed);
    Ok(hashed.iter().cloned().collect::<Vec<u8>>())
}
