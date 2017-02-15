//! Auth
//!
//! Hash and Salt generation
//!
use rand::{Rng, OsRng};
use crypto::bcrypt;
use errors::*;


/// Return a random 128bit salt or error
pub fn new_salt() -> Result<Vec<u8>> {
    const SALT_SIZE: usize = 16;
    let mut salt = [0u8; SALT_SIZE];
    match OsRng::new() {
        Ok(mut rng) => {
            rng.fill_bytes(&mut salt);
        },
        Err(_) => bail!("rng error"),
    };
    Ok(salt.iter().cloned().collect::<Vec<u8>>())
}


/// Return a bcrypt salted hash with provided 'string' and 'salt' or error
pub fn hash(string: &str, salt: &[u8]) -> Result<Vec<u8>> {
    if salt.len() != 16 || (string.len() == 0 || string.len() > 72) {
        bail!("Salt or String error");
    }
    const COST: u32 = 10;
    const OUTPUT_SIZE: usize = 24;
    let mut hashed = [0u8; OUTPUT_SIZE];
    bcrypt::bcrypt(COST, salt, string.as_bytes(), &mut hashed);
    Ok(hashed.iter().cloned().collect::<Vec<u8>>())
}


/// Do a constant time comparison of two hashed byte slices
pub fn const_eq(one: &[u8], two: &[u8]) -> bool {
    let mut ok = true;
    if one.len() != two.len() { ok = false; }
    for (a, b) in one.iter().zip(two.iter()) {
        if a != b { ok = false; }
    }
    ok
}
