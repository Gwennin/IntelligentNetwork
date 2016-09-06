use model::{Link, NewLink, NewInsertableLink, ReadedLink};
use managers::db_manager::*;
use schema::links::dsl::*;
use schema::readed_links::dsl::*;
use diesel::{LoadDsl, FilterDsl, ExpressionMethods, ExecuteDsl};
use diesel::result::Error;
use diesel;
use std::ops::Deref;

impl Link {
    pub fn list_links(space: String) -> Vec<Link> {
        let db = DB_CONNECTION.lock().unwrap();

        let results = links.filter(posted_in.eq(space)).load(db.deref());

        return results.unwrap();
    }

    pub fn add_link(new_link: &NewLink, space: String) -> Vec<Link> {
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

        let inserted = links.filter(id.eq(inserted_id.unwrap())).load(db.deref());
        return inserted.unwrap();
    }

    pub fn delete_link(link_id: i32, space: String) {
        let db = DB_CONNECTION.lock().unwrap();

        diesel::delete(links.filter(id.eq(link_id)).filter(posted_in.eq(space)))
            .execute(db.deref());
    }

    pub fn set_link_read(link_id: i32, by: String) {
        let db = DB_CONNECTION.lock().unwrap();

        let link_readed = ReadedLink {
            read_link: link_id,
            reader: by.to_string(),
        };

        let inserted_id: Result<i32, Error> = diesel::insert(&link_readed)
            .into(readed_links)
            .returning(read_id)
            .get_result(db.deref());
    }

}
