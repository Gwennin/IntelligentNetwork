use model::{Link, NewLink, NewInsertableLink, ReadedLink};
use managers::db_manager::*;
use schema::links::dsl::*;
use schema::readed_links::dsl::*;
use diesel::{LoadDsl, FilterDsl, ExpressionMethods, ExecuteDsl};
use diesel::result::Error;
use diesel::result::Error::{NotFound, DatabaseError};
use diesel::result::DatabaseErrorKind::UniqueViolation;
use diesel;
use std::ops::Deref;
use errors::INError;

impl Link {
    pub fn list_links(space: String) -> Result<Vec<Link>, INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let results = links.filter(posted_in.eq(space)).load(db.deref());

        match results {
            Err(err) => Err(INError::fatal(1, "An error occured while accessing to the database.")),
            Ok(res) => Ok(res),
        }
    }

    pub fn add_link(new_link: &NewLink, space: String) -> Result<Link, INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let new_insertable_link = NewInsertableLink {
            link: new_link.link.to_string(),
            posted_by: new_link.posted_by.to_string(),
            posted_in: space.to_string(),
        };

        let inserted_id: Result<i32, Error> = diesel::insert(&new_insertable_link)
            .into(links)
            .returning(id)
            .get_result(db.deref());

        match inserted_id {
            Err(err) => match err {
                DatabaseError(kind, _) => match kind {
                    UniqueViolation => return Err(INError::new(400, "A link with the same id is allready registered.")),
                    _ => return Err(INError::fatal(1, "An error occured while accessing to the database.")),
                },
                _ => return Err(INError::fatal(1, "An error occured while accessing to the database.")),
            },
            Ok(res) => {
                let inserted: Result<Link, Error> = links.filter(id.eq(inserted_id.unwrap()))
                    .get_result(db.deref());

                match inserted {
                    Err(err) => return Err(INError::fatal(1, "An error occured while accessing to the database.")),
                    Ok(res) => return Ok(res),
                }
            },
        }
    }

    pub fn delete_link(link_id: i32, space: String) -> Option<INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let result : Result<Link, Error> = links
            .filter(id.eq(link_id))
            .get_result(db.deref());

        match result {
            Err(err) => match err {
                NotFound => return Some(INError::new(401, "This link doesn't exist.")),
                _ => return Some(INError::fatal(1, "An error occured while accessing to the database.")),
            },
            _ => {},
        }

        diesel::delete(links.filter(id.eq(link_id)).filter(posted_in.eq(space)))
            .execute(db.deref());

        None
    }

    pub fn set_link_read(link_id: i32, by: String) -> Option<INError> {
        let db = DB_CONNECTION.lock().unwrap();

        let link_readed = ReadedLink {
            read_link: link_id,
            reader: by.to_string(),
        };

        let inserted_id: Result<i32, Error> = diesel::insert(&link_readed)
            .into(readed_links)
            .returning(read_id)
            .get_result(db.deref());

        match inserted_id {
            Err(err) => match err {
                DatabaseError(kind, _) => match kind {
                    UniqueViolation => Some(INError::new(400, "A read link with the same id is allready registered.")),
                    _ => Some(INError::fatal(1, "An error occured while accessing to the database.")),
                },
                NotFound => Some(INError::new(401, "This link doesn't exist.")),
                _ => Some(INError::fatal(1, "An error occured while accessing to the database.")),
            },
            Ok(res) => None,
        }
    }
}
