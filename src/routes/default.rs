use nickel::{Request, Response, MiddlewareResult, MediaType};
use rustc_serialize::json::{ToJson};

use helpers;

pub fn get<'a>(req: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
  res.set(MediaType::Json);

  let response = helpers::status::Response {
    code: 200,
    success: true
  };

  res.send(response.to_json())
}