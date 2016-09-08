
use iron::prelude::{Response, IronResult};
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Header;
use rustc_serialize::json;
use rustc_serialize::Encodable;
use errors::*;

pub struct  ResponseManager;

impl ResponseManager {
    pub fn get_response<T>(content: &Result<T, INError>) -> IronResult<Response>
        where T: Encodable {

        match *content {
            Err(ref err) => {
                let mut http_status = NON_FATAL_HTTP_STATUS;
                if err.fatal {
                    http_status = FATAL_HTTP_STATUS;
                }
                let encoded = json::encode(&err).unwrap();
                Ok(Response::with((http_status, Header(ContentType::json()), encoded)))
            },
            Ok(ref results) => {
                let encoded = json::encode(&results).unwrap();
                Ok(Response::with((status::Ok, Header(ContentType::json()), encoded)))
            },
        }
    }

    pub fn get_response_no_content(error: &Option<INError>) -> IronResult<Response> {
        match *error {
            Some(ref err) => {
                let mut http_status = NON_FATAL_HTTP_STATUS;
                if err.fatal {
                    http_status = FATAL_HTTP_STATUS;
                }
                let encoded = json::encode(&err).unwrap();
                Ok(Response::with((http_status, Header(ContentType::json()), encoded)))
            },
            None => Ok(Response::with((status::NoContent))),
        }
    }

    pub fn get_unauthorized() -> IronResult<Response> {
        Ok(Response::with((status::Unauthorized)))
    }

    pub fn get_forbiden() -> IronResult<Response> {
        Ok(Response::with((status::Forbidden)))
    }

}