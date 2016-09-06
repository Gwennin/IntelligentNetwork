#![allow(unused_variables, unused_must_use)]

extern crate iron;
extern crate router;
extern crate dotenv;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;

extern crate rustc_serialize;
extern crate chrono;

include!(concat!(env!("OUT_DIR"), "/model.rs"));

mod managers;
mod controlers;
mod models;

use managers::server_manager::ServerManager;

fn main() {
    let server = ServerManager::new();
    server.run();
}