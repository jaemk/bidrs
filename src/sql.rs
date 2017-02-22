use postgres::Connection;
use uuid::Uuid;

use models::*;
use errors::*;


pub fn select_user_by_email(conn: &Connection, email: &str) -> Option<User> {
    let qs = "select * from users where email=$1";
    query_or_none!(conn.query(qs, &[&email]), User)
}

pub fn select_user_by_uuid(conn: &Connection, uuid_: &Uuid) -> Option<User> {
    let qs = "select * from users where uuid_=$1 limit 1";
    query_or_none!(conn.query(qs, &[&uuid_]), User)
}

/// Want to delete the auth record and have the users record cascade delete
pub fn delete_user_by_id(conn: &Connection, id: i32) -> Result<()> {
    let qs = "with user_sel as (select auth_id from users where id=$1) \
              delete from auth using user_sel \
              where user_sel.auth_id=auth.id";
    conn.execute(qs, &[&id]).chain_err(|| "Error deleting user/auth")?;
    Ok(())
}


pub fn select_profile_by_user(conn: &Connection, user_id: &i32) -> Option<Profile> {
    let qs = "select * from profiles where user_id=$1 limit 1";
    query_or_none!(conn.query(qs, &[&user_id]), Profile)
}

pub fn select_profile_by_name(conn: &Connection, name: &str) -> Option<Profile> {
    let qs = "select * from profiles where name=$1 limit 1";
    query_or_none!(conn.query(qs, &[&name]), Profile)
}


