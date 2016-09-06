use model::{User, NewUser, UserSpace};
use managers::db_manager::*;
use schema::users::dsl::*;
use schema::user_spaces::dsl::*;
use schema::spaces::dsl::*;
use diesel::{LoadDsl, FilterDsl, ExpressionMethods, ExecuteDsl, SelectDsl};
use diesel::result::Error;
use diesel;
use std::ops::Deref;
use rustc_serialize::{Encodable, Encoder};

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
    pub fn list_users() -> Vec<User> {
        let db = DB_CONNECTION.lock().unwrap();

        let results = users.load(db.deref());

        return results.unwrap();
    }

    pub fn add_user(user: &NewUser) {
        let db = DB_CONNECTION.lock().unwrap();

        let result: Result<String, Error> = diesel::insert(user)
            .into(users)
            .returning(username)
            .get_result(db.deref());
    }

    pub fn delete_user(alias: String) {
        let db = DB_CONNECTION.lock().unwrap();

        diesel::delete(users.filter(username.eq(alias)))
            .execute(db.deref());
    }

    pub fn change_password(alias: String, new_password: String) {
        let db = DB_CONNECTION.lock().unwrap();

        let updated: Result<User, Error> = diesel::update(users
                        .filter(username.eq(alias)))
                        .set(password.eq(new_password))
                        .get_result(db.deref());
    }

    pub fn add_space(user_space: UserSpace) {
        let db = DB_CONNECTION.lock().unwrap();

        let result: Result<i32, Error> = diesel::insert(&user_space)
            .into(user_spaces)
            .returning(id)
            .get_result(db.deref());
    }

    pub fn delete_space(user_space: UserSpace) {
        let db = DB_CONNECTION.lock().unwrap();

        diesel::delete(user_spaces
            .filter(user_id.eq(user_space.user_id))
            .filter(space_id.eq(user_space.space_id)))
            .execute(db.deref());
    }

    pub fn list_user_spaces(alias: String) -> Vec<String> {
        let db = DB_CONNECTION.lock().unwrap();

        let results = user_spaces.select(space_id)
                                .filter(user_id.eq(alias))
                                .load(db.deref());

        return results.unwrap();
    }

    pub fn list_owned_spaces(alias: String) -> Vec<String> {
        let db = DB_CONNECTION.lock().unwrap();

        let results = spaces.select(name)
                                .filter(owner.eq(alias))
                                .filter(public.eq(true))
                                .load(db.deref());

        return results.unwrap();
    }
}
