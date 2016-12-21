#[macro_use] 

extern crate nickel;
extern crate rustc_serialize;

use std::env;
use std::net::SocketAddr;

use nickel::{Nickel, HttpRouter};

mod models;
mod routes;
mod helpers;

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

    server.utilize(router);
    server.listen(server_address).unwrap();
}
