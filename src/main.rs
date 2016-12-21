#[macro_use] extern crate nickel;
extern crate rustc_serialize;

use std::env;
use std::net::SocketAddr;
use nickel::status::StatusCode;

use nickel::{Nickel, JsonBody, HttpRouter, MediaType};
use rustc_serialize::json::{ToJson};

mod models;
mod routes;

fn main() {

    let port : String = env::var("PORT").expect("Missing env var `PORT`");
    let host : String = env::var("HOST").expect("Missing env var `HOST`");


    let mut server = Nickel::new();
    let mut router = Nickel::router();

    router.get("/:last/:first", routes::names::get);

    router.get("/", middleware! { |_, mut response|
        response.set(MediaType::Json);
        r#"{ "foo": "bar" }"#
    });

    router.post("/", middleware! { |request, response|
        let person = try_with!(response, {
            request.json_as::<models::people::Person>().map_err(|e| (StatusCode::BadRequest, e))
        });
        format!("Hello {} {}", person.first_name, person.last_name)
    });

    let server_details : String = format!("{}{}{}", host, ":", port);

    let server_address : SocketAddr = server_details.parse()
        .expect("Unable to parse socket address");

    server.utilize(router);
    server.listen(server_address).unwrap();
}
