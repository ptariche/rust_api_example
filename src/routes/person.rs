use nickel::{Request, Response, MiddlewareResult, JsonBody};
use nickel::status::StatusCode;
use rustc_serialize::json::{ToJson};

use lib;

pub fn get<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
  let first_name = req.param("first").unwrap();
  let last_name = req.param("last").unwrap();

  let person = lib::people::Person {
      first_name: first_name.to_string(),
      last_name: last_name.to_string(),
  };
  res.send(person.to_json())
}

pub fn post<'a>(req: &mut Request, res: Response<'a>) -> MiddlewareResult<'a> {
  let person = try_with!(res, {
      req.json_as::<lib::people::Person>().map_err(|e| (StatusCode::BadRequest, e))
  });
  
  res.send(person.to_json())
}