use iron::prelude::*;
use dotenv::dotenv;
use std::env;
use std::string::String;
use std::str::FromStr;

use managers::routes_manager;

pub struct ServerManager {
    listen_address: String,
    listen_port: u16,
}

impl ServerManager {
    pub fn new() -> ServerManager {
        dotenv().ok();

        let address = env::var("LISTEN_ADDRESS").unwrap();
        let port = u16::from_str(env::var("LISTEN_PORT").unwrap().as_str()).unwrap();

        ServerManager {
            listen_address: address,
            listen_port: port,
        }
    }

    pub fn run(&self) {
        let router = routes_manager::apply_routes();
        
        Iron::new(router).http((self.listen_address.as_str(), self.listen_port)).unwrap();
    }
}