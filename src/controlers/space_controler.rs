use iron::prelude::{Response, IronResult};
use iron::status;
use iron::request::*;
use iron::headers::ContentType;
use iron::modifiers::Header;
use rustc_serialize::json;
use std::io::Read;
use std::str::FromStr;
use router::Router;
use model::*;

pub struct SpaceControler;

impl SpaceControler {
    pub fn list_public_spaces(req: &mut Request) -> IronResult<Response> {
        let spaces = Space::list_public_spaces();

        let encoded = json::encode(&spaces).unwrap();

        let response: IronResult<Response> = Ok(Response::with((status::Ok, Header(ContentType::json()), encoded)));
        return response;
    }

    pub fn add_space(req: &mut Request) -> IronResult<Response> {
        let mut body = String::new();
        req.body.read_to_string(&mut body);

        let space: NewSpace = json::decode(&body).unwrap();
        Space::add_space(&space);

        let response: IronResult<Response> = Ok(Response::with((status::Ok, "")));
        return response;
    }

    pub fn delete_space(req: &mut Request) -> IronResult<Response> {
        let name = req.extensions.get::<Router>().unwrap().find("name").unwrap().to_string();

        Space::delete_space(name);

        let response: IronResult<Response> = Ok(Response::with((status::Ok, "")));
        return response;
    }

    pub fn list_links(req: &mut Request) -> IronResult<Response> {
        let name = req.extensions.get::<Router>().unwrap().find("name").unwrap().to_string();
        let links = Link::list_links(name);

        let encoded = json::encode(&links).unwrap();

        let response: IronResult<Response> = Ok(Response::with((status::Ok, Header(ContentType::json()), encoded)));
        return response;
    }

    pub fn add_link(req: &mut Request) -> IronResult<Response> {
        let name = req.extensions.get::<Router>().unwrap().find("name").unwrap().to_string();

        let mut body = String::new();
        req.body.read_to_string(&mut body);
        let link: NewLink = json::decode(&body).unwrap();

        let inserted = Link::add_link(&link, name);

        let encoded = json::encode(&inserted).unwrap();

        let response: IronResult<Response> = Ok(Response::with((status::Ok, Header(ContentType::json()), encoded)));
        return response;
    }

    pub fn delete_link(req: &mut Request) -> IronResult<Response> {
        let name = req.extensions.get::<Router>().unwrap().find("name").unwrap().to_string();
        let str_id = req.extensions.get::<Router>().unwrap().find("id").unwrap().to_string();
        let link_id: i32 = FromStr::from_str(&str_id).unwrap();

        Link::delete_link(link_id, name);

        let response: IronResult<Response> = Ok(Response::with((status::Ok, "")));
        return response;
    }

    pub fn set_link_read(req: &mut Request) -> IronResult<Response> {
        let user = req.extensions.get::<Router>().unwrap().find("user").unwrap().to_string();
        let str_id = req.extensions.get::<Router>().unwrap().find("id").unwrap().to_string();
        let link_id: i32 = FromStr::from_str(&str_id).unwrap();

        Link::set_link_read(link_id, user);

        let response: IronResult<Response> = Ok(Response::with((status::Ok, "")));
        return response;
    }

}