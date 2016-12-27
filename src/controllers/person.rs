use nickel::{Request, Response, MiddlewareResult, JsonBody, MediaType};
use nickel::status::StatusCode;
use rustc_serialize::json::{ToJson};
use diesel::ExpressionMethods;
use diesel::FilterDsl;
use diesel::LoadDsl;
use diesel;

use helpers;
use lib;
use models;
use uuid::Uuid;

pub fn get<'a>(req: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
  use lib::schema::persons::dsl::{persons, uuid};

  let connection = lib::db::establish_connection();

  let input_uuid = Uuid::parse_str(req.param("uuid").unwrap()).unwrap();
  let results = persons.filter( uuid.eq(input_uuid))
        .load::<models::people::Person>(&connection)
        .expect("error pulling person matching uuid");

  let response;
	if results.len() == 1 {
    let person = &results[0];
    response = helpers::status::Response {
      success: true,
      code: 200,
      data: person.to_json()
    };
  } else {
    let error = helpers::status::Error {
      error : "An error occured attempting to look up the given identifier".to_string(),
    };

    response = helpers::status::Response {
      success: true,
      code: 200,
      data: error.to_json()
    };
  }


  res.set(MediaType::Json);
  res.set(StatusCode::Ok);
  res.send(response.to_json())
}

pub fn post<'a>(req: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
  let connection = lib::db::establish_connection();

  let person = try_with!(res, {
      req.json_as::<models::people::NewPerson>().map_err(|e| (StatusCode::BadRequest, e))
  });


  diesel::insert(&person)
    .into(lib::schema::persons::table)
    .get_result::<models::people::Person>(&connection)
    .expect("create_user_fail");

  let response;

  response = helpers::status::Response {
    success: true,
    code: 201,
    data: person.to_json()
  };


  res.set(MediaType::Json);
  res.set(StatusCode::Ok);
  res.send(response.to_json())
}