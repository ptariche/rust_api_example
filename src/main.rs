#[macro_use] extern crate nickel;
extern crate rustc_serialize;

use nickel::status::StatusCode;
use nickel::{Nickel, JsonBody, HttpRouter, MediaType};
use rustc_serialize::json::{Json, ToJson};

mod models;

fn main() {
    let mut server = Nickel::new();

    // go to http://localhost:6767/your/name to see this route in action
    server.get("/:first/:last", middleware! { |req|
        let first_name = req.param("first").unwrap();
        let last_name = req.param("last").unwrap();

        let person = models::Person {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
        };
        person.to_json()
    });

    server.get("/", middleware! { |_, mut response|
        response.set(MediaType::Json);
        r#"{ "foo": "bar" }"#
    });

    server.post("/", middleware! { |request, response|
        let person = try_with!(response, {
            request.json_as::<models::Person>().map_err(|e| (StatusCode::BadRequest, e))
        });
        format!("Hello {} {}", person.first_name, person.last_name)
    });

    server.listen("127.0.0.1:6767");
}
