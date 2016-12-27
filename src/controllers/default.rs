use nickel::{Request, Response, MiddlewareResult, MediaType};
use std::collections::BTreeMap;
use rustc_serialize::json::{ToJson, Json};
use chrono::{UTC, Local, DateTime};
use nickel::status::StatusCode;

use helpers;

#[derive(RustcDecodable, RustcEncodable)]
struct Data {
  utc_rfc2822: String,
  local_rfc2822: String
}

impl ToJson for Data {
  fn to_json(&self) -> Json {
    let mut map = BTreeMap::new();

    map.insert("utc_rfc2822".to_string(), self.utc_rfc2822.to_json());
    map.insert("local_rfc2822".to_string(), self.local_rfc2822.to_json());

    Json::Object(map)
  }
}

pub fn get<'a>(_req: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {

  let utc_stamp: DateTime<UTC> = UTC::now();
  let local_stamp: DateTime<Local> = Local::now();

  let data = Data {
    utc_rfc2822: utc_stamp.to_rfc2822(),
    local_rfc2822: local_stamp.to_rfc2822(),
  };

  let response = helpers::status::Response {
    success: true,
    code: 200,
    data: data.to_json()
  };

  res.set(MediaType::Json);
  res.set(StatusCode::Ok);
  res.send(response.to_json())
}
