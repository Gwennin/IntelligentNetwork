use chrono::naive::datetime::NaiveDateTime;

use super::schema::users;

#[derive(Queryable, Clone)]
pub struct User {
    pub username: String,
    pub password: String, // En clair pour l'instant mais on peut utiliser rust-crypto
}

#[derive(RustcDecodable)]
#[insertable_into(users)]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

use super::schema::spaces;

#[derive(Queryable, Clone)]
#[insertable_into(spaces)]
pub struct Space {
    pub name: String,
    pub owner: String,
    pub public: bool
}

#[derive(RustcDecodable)]
#[insertable_into(spaces)]
pub struct NewSpace {
    pub name: String,
    pub owner: String,
}

use super::schema::user_spaces;

#[derive(Queryable, Clone)]
#[insertable_into(user_spaces)]
pub struct UserSpace {
    pub user_id: String,
    pub space_id: String,
}

#[derive(Queryable, RustcEncodable, Clone)]
pub struct Link {
    pub link_id: i32,
    pub link: String,
    pub posted_by: String,
    pub posted_in: String,
    pub posted_on: NaiveDateTime
}

use super::schema::links;

#[insertable_into(links)]
pub struct NewInsertableLink {
    pub link: String,
    pub posted_by: String,
    pub posted_in: String,
}

#[derive(RustcDecodable)]
pub struct NewLink {
    pub link: String,
    pub posted_by: String,
}

use super::schema::readed_links;

#[insertable_into(readed_links)]
pub struct ReadedLink {
    pub read_link: i32,
    pub reader: String,
}

