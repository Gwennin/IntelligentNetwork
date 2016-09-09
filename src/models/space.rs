use model::{Space, NewSpace};
use managers::db_manager::*;
use schema::spaces::dsl::*;
use diesel::{LoadDsl, FilterDsl, ExpressionMethods, ExecuteDsl};
use diesel::result::Error;
use diesel::result::Error::{NotFound, DatabaseError};
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel;
use std::ops::Deref;
use rustc_serialize::{Encodable, Encoder};
use errors::INError;

impl Encodable for Space {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("Space", 2, |s| {
            try!(s.emit_struct_field("name", 0, |s| {
                s.emit_str(&self.name)
            }));
            try!(s.emit_struct_field("owner", 1, |s| {
                s.emit_str(&self.owner)
            }));
            Ok(())
        })
    }
}

impl Space {
    pub fn list_public_spaces() -> Result<Vec<Space>, INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let results = spaces.filter(public.eq(true)).load(db.deref());

        match results {
            Err(_) => Err(INError::fatal(1, "An error occured while accessing to the database.")),
            Ok(res) => Ok(res),
        }
    }

    pub fn add_space(space: &NewSpace) -> Option<INError> {
        Space::_add_space(space, true)
    }

    pub fn add_private_space(space: &NewSpace) -> Option<INError> {
        Space::_add_space(space, false)
    }

    fn _add_space(space: &NewSpace, is_public: bool) -> Option<INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let new_space = Space {
            name: space.name.to_string(),
            owner: space.owner.to_string(),
            public: is_public,
        };

        let result: Result<String, Error> = diesel::insert(&new_space)
            .into(spaces)
            .returning(name)
            .get_result(db.deref());

        match result {
            Err(err) => match err {
                DatabaseError(kind, _) => match kind {
                    UniqueViolation => Some(INError::new(300, "This space is allready registered.")),
                    _ => Some(INError::fatal(1, "An error occured while accessing to the database.")),
                },
                _ => Some(INError::fatal(1, "An error occured while accessing to the database.")),
            },
            Ok(_) => None,
        }
    }

    pub fn delete_space(space_name: String) -> Option<INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let result : Result<Space, Error> = spaces
            .filter(name.eq(&space_name))
            .get_result(db.deref());

        match result {
            Err(err) => match err {
                NotFound => return Some(INError::new(301, "This space doesn't exist.")),
                _ => return Some(INError::fatal(1, "An error occured while accessing to the database.")),
            },
            _ => {},
        }

        diesel::delete(spaces.filter(public.eq(true))
            .filter(name.eq(&space_name)))
            .execute(db.deref());

        None
    }
}
