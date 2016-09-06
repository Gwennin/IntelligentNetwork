use std::fmt;
use std::error;
use rustc_serialize::{Encodable, Encoder};
use iron::status::Status;

#[cfg(feature = "teapot")]
pub const NON_FATAL_HTTP_STATUS: Status = Status::ImATeapot;
#[cfg(not(feature = "teapot"))]
pub const NON_FATAL_HTTP_STATUS: Status = Status::BadRequest;

pub const FATAL_HTTP_STATUS: Status = Status::InternalServerError;

#[derive(Debug)]
pub struct INError {
    pub code: u16,
    pub message: String,
    pub fatal: bool // si erreur fatale code HTTP = 500 sinon 400 (418 si feature="teapot")
}

impl fmt::Display for INError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.fatal {
            write!(f, "FATAL ");
        }
        write!(f, "Intelligent Network error n°{}: {}", self.code, self.message)
    }
}

impl error::Error for INError {
    fn description(&self) -> &str {
        return &self.message;
    }

    fn cause(&self) -> Option<&error::Error> {
        return None; // Re-up db errors
    }
}

impl INError {
    pub fn fatal(code: u16, message: &str) -> INError {
        INError {
            code: code,
            message: message.to_string(),
            fatal: true
        }
    }

    pub fn new(code: u16, message: &str) -> INError {
        INError {
            code: code,
            message: message.to_string(),
            fatal: false
        }
    }
}

impl Encodable for INError {
    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
        s.emit_struct("INError", 2, |s| {
            try!(s.emit_struct_field("code", 0, |s| {
                s.emit_u16(self.code)
            }));
            try!(s.emit_struct_field("message", 1, |s| {
                s.emit_str(&self.message)
            }));
            Ok(())
        })
    }
}
