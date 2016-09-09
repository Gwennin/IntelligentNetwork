use model::{User, NewUser, UserSpace};
use managers::db_manager::*;
use schema::users::dsl::*;
use schema::user_spaces::dsl::*;
use schema::spaces::dsl::*;
use diesel::{LoadDsl, FilterDsl, ExpressionMethods, ExecuteDsl, SelectDsl, CountDsl};
use diesel::result::Error;
use diesel::result::Error::{NotFound, DatabaseError};
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel;
use std::ops::Deref;
use rustc_serialize::{Encodable, Encoder};
use errors::INError;

impl Encodable for User {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("User", 1, |s| {
            try!(s.emit_struct_field("username", 0, |s| {
                s.emit_str(&self.username)
            }));
            Ok(())
        })
    }
}

impl User {
    pub fn is_user_exist(alias: String) -> Result<bool, INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let result : Result<i64, Error> = users
            .filter(username.eq(alias))
            .count()
            .get_result(db.deref());

        match result {
            Err(_) => Err(INError::fatal(1, "An error occured while accessing to the database.")),
            Ok(res) => Ok(res == 1),
        }
    }

    pub fn password_match(alias: String, pswd: String) -> Result<bool, INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let result : Result<i64, Error> = users
            .filter(username.eq(alias))
            .filter(password.eq(pswd))
            .count()
            .get_result(db.deref());

        match result {
            Err(_) => Err(INError::fatal(1, "An error occured while accessing to the database.")),
            Ok(res) => Ok(res == 1),
        }
    }

    pub fn list_users() -> Result<Vec<User>, INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let results = users.load(db.deref());

        match results {
            Err(_) => Err(INError::fatal(1, "An error occured while accessing to the database.")),
            Ok(res) => Ok(res),
        }
    }

    pub fn add_user(user: &NewUser) -> Result<String, INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let result = diesel::insert(user)
            .into(users)
            .returning(username)
            .get_result(db.deref());

        match result {
            Err(err) => match err {
                DatabaseError(kind, _) => match kind {
                    UniqueViolation => Err(INError::new(100, "This username is allready taken.")),
                    _ => Err(INError::fatal(1, "An error occured while accessing to the database.")),
                },
                _ => Err(INError::fatal(1, "An error occured while accessing to the database.")),
            },
            Ok(res) => Ok(res),
        }
    }

    pub fn delete_user(alias: String) -> Option<INError>{
        let db = DB_CONNECTION.lock().unwrap();

        let result : Result<User, Error> = users
            .filter(username.eq(alias.to_string()))
            .get_result(db.deref());

        match result {
            Err(err) => match err {
                NotFound => return Some(INError::new(101, "This username doesn't exist.")),
                _ => return Some(INError::fatal(1, "An error occured while accessing to the database.")),
            },
            _ => {},
        }

        diesel::delete(users.filter(username.eq(alias)))
            .execute(db.deref());
        None
    }

    pub fn change_password(alias: String, new_password: String) -> Option<INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let result : Result<User, Error> = users
            .filter(username.eq(alias.to_string()))
            .get_result(db.deref());

        match result {
            Err(err) => match err {
                NotFound => return Some(INError::new(101, "This username doesn't exist.")),
                _ => return Some(INError::fatal(1, "An error occured while accessing to the database.")),
            },
            _ => {},
        }

        let updated: Result<User, Error> = diesel::update(users
                        .filter(username.eq(alias)))
                        .set(password.eq(new_password))
                        .get_result(db.deref());

        match updated {
            Err(_) => Some(INError::fatal(1, "An error occured while accessing to the database.")),
            Ok(_) => None,
        }
    }

    pub fn add_space(user_space: UserSpace) -> Option<INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let result: Result<i32, Error> = diesel::insert(&user_space)
            .into(user_spaces)
            .returning(id)
            .get_result(db.deref());

        match result {
            Err(err) => match err {
                DatabaseError(kind, _) => match kind {
                    UniqueViolation => Some(INError::new(200, "A space with the same name is allready registered.")),
                    _ => return Some(INError::fatal(1, "An error occured while accessing to the database.")),
                },
                _ => return Some(INError::fatal(1, "An error occured while accessing to the database.")),
            },
            Ok(_) => None,
        }
    }

    pub fn delete_space(user_space: UserSpace) -> Option<INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let result : Result<String, Error> = user_spaces.select(space_id)
            .filter(user_id.eq(&user_space.user_id))
            .filter(space_id.eq(&user_space.space_id))
            .get_result(db.deref());

        match result {
            Err(err) => match err {
                NotFound => return Some(INError::new(201, "This user doesn't have any registration for this space")),
                _ => return Some(INError::fatal(1, "An error occured while accessing to the database.")),
            },
            _ => {},
        }

        diesel::delete(user_spaces
            .filter(user_id.eq(&user_space.user_id))
            .filter(space_id.eq(&user_space.space_id)))
            .execute(db.deref());

        None
    }

    pub fn list_user_spaces(alias: String) -> Result<Vec<String>, INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let results = user_spaces.select(space_id)
                                .filter(user_id.eq(alias))
                                .load(db.deref());

        match results {
            Err(_) => Err(INError::fatal(1, "An error occured while accessing to the database.")),
            Ok(res) => Ok(res),
        }
    }

    pub fn list_owned_spaces(alias: String) -> Result<Vec<String>, INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let results = spaces.select(name)
                                .filter(owner.eq(alias))
                                .filter(public.eq(true))
                                .load(db.deref());

        match results {
            Err(_) => Err(INError::fatal(1, "An error occured while accessing to the database.")),
            Ok(res) => Ok(res),
        }
    }
}
