use nickel::{Request, Response, MiddlewareResult};

use rustc_serialize::json::{ToJson};

use models;

pub fn get<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
  let first_name = req.param("first").unwrap();
  let last_name = req.param("last").unwrap();

  let person = models::people::Person {
      first_name: first_name.to_string(),
      last_name: last_name.to_string(),
  };
  res.send(person.to_json())
}