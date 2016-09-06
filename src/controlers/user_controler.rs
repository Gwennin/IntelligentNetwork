use iron::prelude::{Response, IronResult};
use iron::status;
use iron::request::*;
use iron::headers::ContentType;
use iron::modifiers::Header;
use rustc_serialize::json;
use std::io::Read;
use router::Router;
use model::*;

pub struct UserControler;

impl UserControler {
    pub fn list_users(req: &mut Request) -> IronResult<Response> {

        let users = User::list_users();

        let encoded = json::encode(&users).unwrap();

        let response: IronResult<Response> = Ok(Response::with((status::Ok, Header(ContentType::json()), encoded)));
        return response;
    }

    pub fn add_user(req: &mut Request) -> IronResult<Response> {
        let mut body = String::new();
        req.body.read_to_string(&mut body);

        let user: NewUser = json::decode(&body).unwrap();
        let space = NewSpace {
            name: "u_".to_string() + &user.username,
            owner: user.username.to_string()
        };

        User::add_user(&user);
        Space::add_private_space(&space);

        let response: IronResult<Response> = Ok(Response::with((status::Ok, "")));
        return response;
    }

    pub fn delete_user(req: &mut Request) -> IronResult<Response> {
        
        let alias = req.extensions.get::<Router>().unwrap().find("alias").unwrap().to_string();

        User::delete_user(alias);

        let response: IronResult<Response> = Ok(Response::with((status::Ok, "")));
        return response;
    }

    pub fn change_password(req: &mut Request) -> IronResult<Response> {
        
        let alias = req.extensions.get::<Router>().unwrap().find("alias").unwrap().to_string();

        let mut password = String::new();
        req.body.read_to_string(&mut password);
        
        User::change_password(alias, password);
        
        let response: IronResult<Response> = Ok(Response::with((status::Ok, "")));
        return response;
    }

    pub fn spaces(req: &mut Request) -> IronResult<Response> {
        
        let alias = req.extensions.get::<Router>().unwrap().find("alias").unwrap().to_string();

        let spaces = User::list_user_spaces(alias);
        let encoded = json::encode(&spaces).unwrap();

        let response: IronResult<Response> = Ok(Response::with((status::Ok, Header(ContentType::json()), encoded)));
        return response;
    }

    pub fn add_space(req: &mut Request) -> IronResult<Response> {
        
        let alias = req.extensions.get::<Router>().unwrap().find("alias").unwrap().to_string();
        let space = req.extensions.get::<Router>().unwrap().find("space").unwrap().to_string();

        let user_space = UserSpace {
            user_id: alias,
            space_id: space
        };

        User::add_space(user_space);

        let response: IronResult<Response> = Ok(Response::with((status::Ok, "")));
        return response;
    }

    pub fn delete_space(req: &mut Request) -> IronResult<Response> {
        
        let alias = req.extensions.get::<Router>().unwrap().find("alias").unwrap().to_string();
        let space = req.extensions.get::<Router>().unwrap().find("space").unwrap().to_string();

        let user_space = UserSpace {
            user_id: alias,
            space_id: space
        };

        User::delete_space(user_space);

        let response: IronResult<Response> = Ok(Response::with((status::Ok, "")));
        return response;
    }

    pub fn owned_spaces(req: &mut Request) -> IronResult<Response> {
        
        let alias = req.extensions.get::<Router>().unwrap().find("alias").unwrap().to_string();

        let spaces = User::list_owned_spaces(alias);
        let encoded = json::encode(&spaces).unwrap();

        let response: IronResult<Response> = Ok(Response::with((status::Ok, Header(ContentType::json()), encoded)));
        return response;
    }
}