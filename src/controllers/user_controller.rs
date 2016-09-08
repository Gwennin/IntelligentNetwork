use iron::prelude::{Response, IronResult};
use iron::request::*;
use rustc_serialize::json;
use std::io::Read;
use model::*;
use managers::response_manager::ResponseManager;
use managers::request_manager::RequestManager;
use controllers::login_controller::LoginController;
use managers::session_manager::SessionManager;

pub struct UserController;

impl UserController {
    pub fn list_users(req: &mut Request) -> IronResult<Response> {
        if !LoginController::is_token_valid(req) {
            return ResponseManager::get_unauthorized();
        }

        let users = User::list_users();

        ResponseManager::get_response(&users)
    }

    pub fn add_user(req: &mut Request) -> IronResult<Response> {
        let mut body = String::new();
        req.body.read_to_string(&mut body);

        let user: NewUser = json::decode(&body).unwrap();
        let space = NewSpace {
            name: "u_".to_string() + &user.username,
            owner: user.username.to_string()
        };

        let new_user = User::add_user(&user);

        match new_user {
            Err(ref err) => return ResponseManager::get_response(&new_user),
            _ => {}
        }

        let result = Space::add_private_space(&space);
        ResponseManager::get_response_no_content(&result)
    }

    pub fn delete_user(req: &mut Request) -> IronResult<Response> {
        if !LoginController::is_token_valid(req) {
            return ResponseManager::get_unauthorized();
        }

        let alias = RequestManager::extract_url_part(req, "alias").unwrap();
        let token = RequestManager::extract_token(req).unwrap();

        if SessionManager::get_session_user(&token) != Some(alias.clone()) {
            return ResponseManager::get_forbiden();
        }

        let result = User::delete_user(alias);
        ResponseManager::get_response_no_content(&result)
    }

    pub fn change_password(req: &mut Request) -> IronResult<Response> {
        if !LoginController::is_token_valid(req) {
            return ResponseManager::get_unauthorized();
        }

        let alias = RequestManager::extract_url_part(req, "alias").unwrap();
        let token = RequestManager::extract_token(req).unwrap();

        if SessionManager::get_session_user(&token) != Some(alias.clone()) {
            return ResponseManager::get_forbiden();
        }

        let mut password = String::new();
        req.body.read_to_string(&mut password);
        
        let result = User::change_password(alias, password);
        return ResponseManager::get_response_no_content(&result);
    }

    pub fn spaces(req: &mut Request) -> IronResult<Response> {
        if !LoginController::is_token_valid(req) {
            return ResponseManager::get_unauthorized();
        }

        let alias = RequestManager::extract_url_part(req, "alias").unwrap();
        let token = RequestManager::extract_token(req).unwrap();

        if SessionManager::get_session_user(&token) != Some(alias.clone()) {
            return ResponseManager::get_forbiden();
        }

        let spaces = User::list_user_spaces(alias);
        ResponseManager::get_response(&spaces)
    }

    pub fn add_space(req: &mut Request) -> IronResult<Response> {
        if !LoginController::is_token_valid(req) {
            return ResponseManager::get_unauthorized();
        }

        let alias = RequestManager::extract_url_part(req, "alias").unwrap();
        let space = RequestManager::extract_url_part(req, "space").unwrap();
        let token = RequestManager::extract_token(req).unwrap();

        if SessionManager::get_session_user(&token) != Some(alias.clone()) {
            return ResponseManager::get_forbiden();
        }

        let user_space = UserSpace {
            user_id: alias,
            space_id: space
        };

        let result = User::add_space(user_space);
        ResponseManager::get_response_no_content(&result)
    }

    pub fn delete_space(req: &mut Request) -> IronResult<Response> {
        if !LoginController::is_token_valid(req) {
            return ResponseManager::get_unauthorized();
        }

        let alias = RequestManager::extract_url_part(req, "alias").unwrap();
        let space = RequestManager::extract_url_part(req, "space").unwrap();
        let token = RequestManager::extract_token(req).unwrap();

        if SessionManager::get_session_user(&token) != Some(alias.clone()) {
            return ResponseManager::get_forbiden();
        }

        let user_space = UserSpace {
            user_id: alias,
            space_id: space
        };

        let result = User::delete_space(user_space);
        ResponseManager::get_response_no_content(&result)
    }

    pub fn owned_spaces(req: &mut Request) -> IronResult<Response> {
        if !LoginController::is_token_valid(req) {
            return ResponseManager::get_unauthorized();
        }

        let alias = RequestManager::extract_url_part(req, "alias").unwrap();
        let token = RequestManager::extract_token(req).unwrap();

        if SessionManager::get_session_user(&token) != Some(alias.clone()) {
            return ResponseManager::get_forbiden();
        }

        let spaces = User::list_owned_spaces(alias);
        ResponseManager::get_response(&spaces)
    }
}