#![feature(proc_macro)]

#[macro_use] extern crate nickel;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

extern crate dotenv;
extern crate rustc_serialize;
extern crate hyper;
extern crate chrono;
extern crate uuid;

use std::env;
use std::net::SocketAddr;
use nickel::{Nickel, HttpRouter, NickelError, Action, Request};

mod lib;
mod models;
mod controllers;
mod helpers;
mod middleware;


const DEFAULT_HOST: &'static str = "127.0.0.1";
const DEFAULT_PORT: &'static str = "4001";

fn main() {
    dotenv::dotenv().ok();

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/", controllers::default::get);
    router.get("/person/:uuid/lookup", controllers::person::get);
    router.put("/person/:uuid/update", controllers::person::put);
    router.delete("/person/:uuid/destroy", controllers::person::delete);
    router.post("/person/create", controllers::person::post);

    let custom_handler: fn(&mut NickelError<()>, &mut Request<()>) -> Action = middleware::errors::enable_handler;

    server.handle_error(custom_handler);

    server.utilize(middleware::logs::enable_logs);
    server.utilize(middleware::cors::enable_cors);
    server.utilize(router);

	let port = match env::var("PORT") {
		Ok(value) => value,
		Err(_) => DEFAULT_PORT.to_string(),
    };

	let host = match env::var("HOST") {
		Ok(value) => value,
		Err(_) => DEFAULT_HOST.to_string(),
    };

    let server_details : String = format!("{}{}{}", host, ":", port);
    let server_address : SocketAddr = server_details.parse()
        .expect("Unable to parse socket address");

    server.listen(server_address).unwrap();
}
