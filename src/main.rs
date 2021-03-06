extern crate iron;
extern crate router;
extern crate dotenv;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate diesel;

extern crate rustc_serialize;
extern crate chrono;
extern crate uuid;

include!(concat!(env!("OUT_DIR"), "/model.rs"));

mod managers;
mod controllers;
mod models;
mod errors;
mod timer;

use managers::server_manager::ServerManager;

fn main() {
    let server = ServerManager::new();
    server.run();
}