#[macro_use] 

extern crate nickel;
extern crate rustc_serialize;
extern crate hyper;

use std::env;
use std::net::SocketAddr;
use nickel::{Nickel, HttpRouter};

mod lib;
mod routes;
mod helpers;
mod middleware;

fn main() {

    let port : String = env::var("PORT").expect("Missing env var `PORT`");
    let host : String = env::var("HOST").expect("Missing env var `HOST`");

    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/", routes::default::get);
    router.get("/:last/:first", routes::person::get);
    router.post("/person/create", routes::person::post);

    let server_details : String = format!("{}{}{}", host, ":", port);

    let server_address : SocketAddr = server_details.parse()
        .expect("Unable to parse socket address");

    server.utilize(middleware::cors::enable_cors);
    server.utilize(middleware! { |request|
        println!("Method: {:?} | Path: {:?} | Source: {:?}", request.origin.method, request.origin.uri, request.origin.remote_addr);
    });
    server.utilize(router);
    server.listen(server_address).unwrap();
}
