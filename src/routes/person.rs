use nickel::{Request, Response, MiddlewareResult, JsonBody, MediaType};
use nickel::status::StatusCode;
use rustc_serialize::json::{ToJson};

use helpers;
use lib;

pub fn get<'a>(req: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
  let first_name = req.param("first").unwrap();
  let last_name = req.param("last").unwrap();

  let person = lib::people::Person {
      first_name: first_name.to_string(),
      last_name: last_name.to_string(),
  };

  let response = helpers::status::Response {
    success: true,
    code: 200,
    data: person.to_json()
  };

  res.set(MediaType::Json);
  res.set(StatusCode::Ok);
  res.send(response.to_json())
}

pub fn post<'a>(req: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
  let person = try_with!(res, {
      req.json_as::<lib::people::Person>().map_err(|e| (StatusCode::BadRequest, e))
  });

  let response = helpers::status::Response {
    success: true,
    code: 201,
    data: person.to_json()
  };

  res.set(MediaType::Json);
  res.set(StatusCode::Ok);
  res.send(response.to_json())
}