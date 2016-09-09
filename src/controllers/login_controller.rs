use iron::prelude::{Response, IronResult};
use iron::request::*;
use iron::headers::{Authorization, Basic};
use managers::session_manager::SessionManager;
use chrono::duration::Duration;
use chrono::offset::utc::UTC;
use std::env;
use std::str::FromStr;
use std::ops::Add;
use managers::response_manager::ResponseManager;
use managers::request_manager::RequestManager;
use managers::authentication_manager::AuthenticationManager;
use errors::INError;
use model::AuthResult;

pub struct LoginController;

impl LoginController {
    pub fn login(req: &mut Request) -> IronResult<Response> {
        let auth_header = req.headers.get::<Authorization<Basic>>();

        match auth_header {
            Some(header) => {
                let authenticator = AuthenticationManager::get_authenticator();

                if let Some(auth) = authenticator {
                    let username = header.username.clone();

                    if let Some(password) = header.password.clone() {
                        if auth.authenticate(username.clone(), password) {
                            let token = SessionManager::open_session(&username);

                            let auth_result = AuthResult {
                                token: token,
                            };
                            return ResponseManager::get_response(&Ok(auth_result));
                        }
                    }
                }

                return ResponseManager::get_unauthorized();
            },
            None => {
                let error = INError::new(2, "No authorization header found.");
                return ResponseManager::get_response_no_content(&Some(error));
            },
        }
    }

    pub fn is_token_valid(req: &mut Request) -> bool {
        let opt_token = RequestManager::extract_token(req);

        if let Some(token) = opt_token {
            let date = SessionManager::get_session_opened_date(&token.clone());

            if let Some(opened_on) = date {
                let session_timeout = env::var("SESSION_TIMEOUT").unwrap();
                let seconds = i64::from_str(&session_timeout).unwrap();

                let expire_date = opened_on.add(Duration::seconds(seconds));
                let current_date = UTC::now();

                if current_date.lt(&expire_date) {
                    return true;
                }

                SessionManager::close_session(&token);
            }
        }
        
        return false;
    }

    pub fn logout(req: &mut Request) -> IronResult<Response> {
        if !Self::is_token_valid(req) {
            return ResponseManager::get_unauthorized();
        }

        let opt_token = RequestManager::extract_token(req);

        match opt_token {
            Some(token) => {
                SessionManager::close_session(&token);
                return ResponseManager::get_response_no_content(&None);
            },
            None => {
                let error = INError::new(2, "No authorization header found.");
                return ResponseManager::get_response_no_content(&Some(error));
            },
        }
    }
}
