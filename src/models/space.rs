use model::{Space, NewSpace};
use managers::db_manager::*;
use schema::spaces::dsl::*;
use diesel::{LoadDsl, FilterDsl, ExpressionMethods, ExecuteDsl};
use diesel::result::Error;
use diesel;
use std::ops::Deref;
use rustc_serialize::{Encodable, Encoder};

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
    pub fn list_public_spaces() -> Vec<Space> {
        let db = DB_CONNECTION.lock().unwrap();

        let results = spaces.filter(public.eq(true)).load(db.deref());

        return results.unwrap();
    }

    pub fn add_space(space: &NewSpace) {
        let db = DB_CONNECTION.lock().unwrap();

        let result: Result<String, Error> = diesel::insert(space)
            .into(spaces)
            .returning(name)
            .get_result(db.deref());
    }

    pub fn add_private_space(space: &NewSpace) {
        let db = DB_CONNECTION.lock().unwrap();

        let new_space = Space {
            name: space.name.to_string(),
            owner: space.owner.to_string(),
            public: false,
        };

        let result: String = diesel::insert(&new_space)
            .into(spaces)
            .returning(name)
            .get_result(db.deref()).unwrap();
    }

    pub fn delete_space(space_name: String) {
        let db = DB_CONNECTION.lock().unwrap();

        diesel::delete(spaces.filter(public.eq(true)).filter(name.eq(space_name)))
            .execute(db.deref());
    }
}
