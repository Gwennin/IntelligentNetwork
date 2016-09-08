use std::env;

pub trait Authenticator {
    //fn initialize() -> Box<Authenticator>;
    fn authenticate(&self, username: String, password: String) -> bool;
}

pub struct AuthenticationManager;

impl AuthenticationManager {
    pub fn get_authenticator() -> Option<Box<Authenticator>> {

        let auth_type = env::var("AUTH_TYPE").unwrap();
        match auth_type.as_ref() {
            "DB" => Some(DBAuthenticator::initialize()),
            _ => None
        }
    }
}

use model::User;

struct DBAuthenticator;

impl DBAuthenticator {
      fn initialize() -> Box<Authenticator> {
        return Box::new(DBAuthenticator {});
    }
}

impl Authenticator for DBAuthenticator {
    fn authenticate(&self, username: String, password: String) -> bool {
        let result = User::password_match(username, password);

        match result {
            Ok(matching) => matching,
            Err(err) => false,
        }
    }
}