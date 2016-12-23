#[macro_use] 
extern crate nickel;

extern crate rustc_serialize;
extern crate hyper;
extern crate chrono;

use std::env;
use std::net::SocketAddr;
use nickel::{Nickel, HttpRouter, NickelError, Action, Request};

mod lib;
mod routes;
mod helpers;
mod middleware;

fn main() {
    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/", routes::default::get);
    router.get("/:last/:first", routes::person::get);
    router.post("/person/create", routes::person::post);

    let custom_handler: fn(&mut NickelError<()>, &mut Request<()>) -> Action = middleware::errors::enable_handler;

    server.handle_error(custom_handler);

    server.utilize(middleware::logs::enable_logs);
    server.utilize(middleware::cors::enable_cors);
    server.utilize(router);

    let port : String = env::var("PORT").expect("Missing env var `PORT`");
    let host : String = env::var("HOST").expect("Missing env var `HOST`");
    let server_details : String = format!("{}{}{}", host, ":", port);
    let server_address : SocketAddr = server_details.parse()
        .expect("Unable to parse socket address");

    server.listen(server_address).unwrap();
}
