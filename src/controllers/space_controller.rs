use iron::prelude::{Response, IronResult};
use iron::request::*;
use rustc_serialize::json;
use std::io::Read;
use std::str::FromStr;
use router::Router;
use model::*;
use managers::response_manager::ResponseManager;

pub struct SpaceController;

impl SpaceController {
    pub fn list_public_spaces(req: &mut Request) -> IronResult<Response> {
        let spaces = Space::list_public_spaces();

        ResponseManager::get_response(&spaces)
    }

    pub fn add_space(req: &mut Request) -> IronResult<Response> {
        let mut body = String::new();
        req.body.read_to_string(&mut body);

        let space: NewSpace = json::decode(&body).unwrap();

        let result = Space::add_space(&space);
        ResponseManager::get_response_no_content(&result)
    }

    pub fn delete_space(req: &mut Request) -> IronResult<Response> {
        let name = req.extensions.get::<Router>().unwrap().find("name").unwrap().to_string();

        let result = Space::delete_space(name);
        ResponseManager::get_response_no_content(&result)
    }

    pub fn list_links(req: &mut Request) -> IronResult<Response> {
        let name = req.extensions.get::<Router>().unwrap().find("name").unwrap().to_string();
        let links = Link::list_links(name);
        ResponseManager::get_response(&links)
    }

    pub fn add_link(req: &mut Request) -> IronResult<Response> {
        let name = req.extensions.get::<Router>().unwrap().find("name").unwrap().to_string();

        let mut body = String::new();
        req.body.read_to_string(&mut body);
        let link: NewLink = json::decode(&body).unwrap();

        let inserted = Link::add_link(&link, name);
        ResponseManager::get_response(&inserted)
    }

    pub fn delete_link(req: &mut Request) -> IronResult<Response> {
        let name = req.extensions.get::<Router>().unwrap().find("name").unwrap().to_string();
        let str_id = req.extensions.get::<Router>().unwrap().find("id").unwrap().to_string();
        let link_id: i32 = FromStr::from_str(&str_id).unwrap();

        let result = Link::delete_link(link_id, name);
        ResponseManager::get_response_no_content(&result)
    }

    pub fn set_link_read(req: &mut Request) -> IronResult<Response> {
        let user = req.extensions.get::<Router>().unwrap().find("user").unwrap().to_string();
        let str_id = req.extensions.get::<Router>().unwrap().find("id").unwrap().to_string();
        let link_id: i32 = FromStr::from_str(&str_id).unwrap();

        let result = Link::set_link_read(link_id, user);
        ResponseManager::get_response_no_content(&result)
    }
}
