use iron::prelude::{Response, IronResult};
use iron::request::*;
use rustc_serialize::json;
use std::io::Read;
use std::str::FromStr;
use model::*;
use managers::response_manager::ResponseManager;
use managers::request_manager::RequestManager;
use controllers::login_controller::LoginController;
use managers::session_manager::SessionManager;

pub struct SpaceController;

impl SpaceController {
    pub fn list_public_spaces(req: &mut Request) -> IronResult<Response> {
        if !LoginController::is_token_valid(req) {
            return ResponseManager::get_unauthorized();
        }

        let spaces = Space::list_public_spaces();

        ResponseManager::get_response(&spaces)
    }

    pub fn add_space(req: &mut Request) -> IronResult<Response> {
        if !LoginController::is_token_valid(req) {
            return ResponseManager::get_unauthorized();
        }

        let mut body = String::new();
        req.body.read_to_string(&mut body);

        let space: NewSpace = json::decode(&body).unwrap();

        let result = Space::add_space(&space);
        ResponseManager::get_response_no_content(&result)
    }

    pub fn delete_space(req: &mut Request) -> IronResult<Response> {
        if !LoginController::is_token_valid(req) {
            return ResponseManager::get_unauthorized();
        }

        let name = RequestManager::extract_url_part(req, "name").unwrap();

        let result = Space::delete_space(name);
        ResponseManager::get_response_no_content(&result)
    }

    pub fn list_links(req: &mut Request) -> IronResult<Response> {
        if !LoginController::is_token_valid(req) {
            return ResponseManager::get_unauthorized();
        }

        let name = RequestManager::extract_url_part(req, "name").unwrap();
        let token = RequestManager::extract_token(req).unwrap();
        let user = SessionManager::get_session_user(&token).unwrap();

        let links = Link::list_links(name, user);
        ResponseManager::get_response(&links)
    }

    pub fn add_link(req: &mut Request) -> IronResult<Response> {
        if !LoginController::is_token_valid(req) {
            return ResponseManager::get_unauthorized();
        }

        let name = RequestManager::extract_url_part(req, "name").unwrap();

        let mut body = String::new();
        req.body.read_to_string(&mut body);
        let link: NewLink = json::decode(&body).unwrap();

        let inserted = Link::add_link(&link, name);
        ResponseManager::get_response(&inserted)
    }

    pub fn delete_link(req: &mut Request) -> IronResult<Response> {
        if !LoginController::is_token_valid(req) {
            return ResponseManager::get_unauthorized();
        }

        let name = RequestManager::extract_url_part(req, "name").unwrap();
        let str_id = RequestManager::extract_url_part(req, "id").unwrap();
        let link_id: i32 = FromStr::from_str(&str_id).unwrap();

        let result = Link::delete_link(link_id, name);
        ResponseManager::get_response_no_content(&result)
    }

    pub fn set_link_read(req: &mut Request) -> IronResult<Response> {
        if !LoginController::is_token_valid(req) {
            return ResponseManager::get_unauthorized();
        }

        let str_id = RequestManager::extract_url_part(req, "id").unwrap();
        let link_id: i32 = FromStr::from_str(&str_id).unwrap();
        let token = RequestManager::extract_token(req).unwrap();
        let user = SessionManager::get_session_user(&token).unwrap();

        let result = Link::set_link_read(link_id, user);
        ResponseManager::get_response_no_content(&result)
    }
}
