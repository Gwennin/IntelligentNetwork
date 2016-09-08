use iron::request::*;
use iron::headers::{Authorization, Bearer};
use router::Router;

pub struct RequestManager;

impl RequestManager {
    pub fn extract_url_part(req: &Request, identifier: &str) -> Option<String> {
        let item = req.extensions.get::<Router>().unwrap().find(identifier);

        match item {
            Some(part) => Some(part.to_string()),
            None => None,
        }
    }

    pub fn extract_token(req: &Request) -> Option<String> {
        let auth_header = req.headers.get::<Authorization<Bearer>>();

        if let Some(header) = auth_header {
            let token = header.token.clone();
            return Some(token);
        }

        return None;
    }
}