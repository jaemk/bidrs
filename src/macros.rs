//! Project macros
//!


#[macro_export]
/// Intended for inserting a row and returning
/// a Result with either a full instance of the table (model::*)
/// entry or an error-chain error describing the insertion failure.
///
/// # Examples
///
/// ```rust,ignore
/// impl NewUser {
///     ...
///     pub fn create(self, conn: &Connection) -> Result<User> {
///         let qs = "insert into users (auth_id, email, uuid_) values ($1, $2, $3) \
///                   returning id, date_created, date_modified";
///         try_insert_to_model!(conn.query(qs, &[&self.auth_id, &self.email, &self.uuid_]) ; // - insert query
///                              User ;                                                       // - models:: struct to create
///                              id: 0, date_created: 1, date_modified: 2 ;                   // - struct fields to be stripped
///                                                                                           //   from row 'returning' row
///                              auth_id: self.auth_id, email: self.email, uuid_: self.uuid_) // - remaining struct fields to
///                                                                                           //   be populated from input struct
///     }
/// }
/// ```
macro_rules! try_insert_to_model {
    ($query:expr ; $model:ident ;
     $($rowvar:ident : $rowindex:expr),* ;
     $($var:ident : $arg:expr),*) => {
        match $query {
            Ok(rows) => {
                match rows.iter().next() {
                    Some(row) => Ok($model {
                        $(
                            $rowvar: row.get($rowindex),
                         )*
                        $(
                            $var : $arg,
                         )*
                    }),
                    _ => bail!("return error")
                }
            }
            Err(postgres::error::Error::Db(err)) => {
                let message = err.message.to_string();
                bail!(err.detail.unwrap_or("no details".to_string()) + " | " +
                    message.as_str())
            }
            _ => bail!("Conversion or IO error".to_string())
        }
    }
}


#[macro_export]
/// Intended for pulling out the first row of
/// a query, returning a populated model-struct in an Option
///
/// # Examples
///
/// ```rust,ignore
/// pub fn select_user_by_id(conn: &Connection, user_id: &i32) -> Option<User> {
///     let qs = "select id, username, uuid_, date_created, date_modified \
///               from user_ where id = $1";
///     query_or_none!(conn.query(qs, &[user_id]), User)
/// }
/// ```
macro_rules! query_or_none {
    ($query:expr, $model:ident) => {
        match $query.unwrap().iter().next() {
            Some(row) => Some($model::from_row(row)),
            _ => None,
        }
    }
}


#[macro_export]
/// Intended for rolling the all the rows of a query result into a vec over the
/// designated model-struct.
///
/// # Examples
///
/// ```rust,ignore
/// pub fn select_users_all(conn: &Connection) -> Vec<User> {
///     let qs = "select id, username, uuid_, date_created, date_modified \
///               from user_";
///     query_coll!(conn.query(qs, &[]), User)
/// }
/// ```
macro_rules! query_coll {
    ($query:expr, $model:ident) => {
        $query.unwrap().iter()
              .map(|row| $model::from_row(row))
              .collect::<Vec<_>>()
    }
}


#[macro_export]
/// Try an expr, return early with a provided error or default
/// to status::InternalServerError
macro_rules! try_server_error {
    ( $exp: expr ) => {
        match $exp {
            Ok(ok) => ok,
            Err(_) => return Ok(Response::with(
                    (status::InternalServerError, "unknown error")
                    ))
        }
    };
    ( $exp: expr, $msg: expr ) => {
        match $exp {
            Ok(ok) => ok,
            Err(_) => return Ok(Response::with(
                    (status::InternalServerError, $msg)
                    ))
        }
    };
    ( $exp: expr ; $error: expr ) => {
        match $exp {
            Ok(ok) => ok,
            Err(err) => return Ok(Response::with(
                    ($error, err.description())
                    ))
        }
    }
}


#[macro_export]
/// HashSet literals
///
/// # Examples
///
/// ```rust,ignore
/// let set = hashset!("one", "two");
/// assert!(set.contains("one"));
/// assert_eq!(set.contains("three"), false);
/// ```
macro_rules! hashset {
    ($($val: expr),*) => {
        {
            let mut set = ::std::collections::HashSet::new();
            $(
                set.insert($val);
             )*
            set
        }
    }
}

