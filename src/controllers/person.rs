use nickel::{Request, Response, MiddlewareResult, JsonBody, MediaType};
use nickel::status::StatusCode;
use rustc_serialize::json::{ToJson, Json};
use std::collections::BTreeMap;
use diesel::ExpressionMethods;
use diesel::FilterDsl;
use diesel::ExecuteDsl;
use diesel::LoadDsl;
use diesel;

use helpers;
use lib;
use models;
use uuid::Uuid;


fn validate_get_delete (u: &str) -> BTreeMap<String, Vec<String>> {
  let mut validations: BTreeMap<String, Vec<String>> = BTreeMap::new();

  validations.insert("uuid".to_string(), vec![u.to_string(), helpers::validator::ValidTypes::Uuid.to_string()]);
  let result = helpers::validator::validate_map(validations);

  result
}

pub fn get<'a>(req: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
  use lib::schema::persons::dsl::{persons, uuid};

  let validation: BTreeMap<String, Vec<String>> = validate_get_delete(req.param("uuid").unwrap());
  let validation_json = validation.clone();
  let is_valid = helpers::validator::has_errors(validation);

  let response;

  if is_valid {
    let connection = lib::db::establish_connection();

    let input_uuid = Uuid::parse_str(req.param("uuid").unwrap()).unwrap();
    let results = persons.filter( uuid.eq(input_uuid))
      .load::<models::people::Person>(&connection)
      .expect("error pulling person matching uuid");

    if results.len() == 1 {
      let person = &results[0];
      response = helpers::status::Response {
        success: true,
        code: 200,
        data: person.to_json()
      };

      res.set(StatusCode::Ok);

    } else {
      let error = helpers::status::Error {
        error : "An error occured attempting to look up the given identifier".to_string(),
        case: Json::from_str("{}").unwrap()
      };

      response = helpers::status::Response {
        success: false,
        code: 400,
        data: error.to_json()
      };

      res.set(StatusCode::BadRequest);

    }
  } else {
    let case = format!("{:?}", validation_json);
    let error = helpers::status::Error {
      error : "There is one or more validation error".to_string(),
      case: Json::from_str(&case).unwrap()
    };

    response = helpers::status::Response {
      success: false,
      code: 412,
      data: error.to_json()
    };
    res.set(StatusCode::PreconditionFailed); 
  }

  res.set(MediaType::Json);
  res.send(response.to_json())
}


fn validate_put (t: &models::people::UpdatePerson, u: &str) -> BTreeMap<String, Vec<String>> {
  let mut validations: BTreeMap<String, Vec<String>> = BTreeMap::new();

  validations.insert("uuid".to_string(), vec![u.to_string(), helpers::validator::ValidTypes::Uuid.to_string()]);
  validations.insert("first_name".to_string(), vec![t.first_name.to_string(), helpers::validator::ValidTypes::AlphaNumberic.to_string()]);
  validations.insert("last_name".to_string(), vec![t.last_name.to_string(), helpers::validator::ValidTypes::AlphaNumberic.to_string()]);

  let result = helpers::validator::validate_map(validations);

  result
}

pub fn put<'a>(req: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
  use lib::schema::persons::dsl::{persons, uuid};

  let person = try_with!(res, {
      req.json_as::<models::people::UpdatePerson>().map_err(|e| (StatusCode::BadRequest, e))
  });

  let validation: BTreeMap<String, Vec<String>> = validate_put(&person, req.param("uuid").unwrap());
  let validation_json = validation.clone();
  let is_valid = helpers::validator::has_errors(validation);

  let response;

  if is_valid {
    let connection = lib::db::establish_connection();

    let input_uuid = Uuid::parse_str(req.param("uuid").unwrap()).unwrap();
    let results = persons.filter( uuid.eq(input_uuid))
      .load::<models::people::Person>(&connection)
      .expect("error pulling person matching uuid");

    if results.len() == 1 {
      let result = diesel::update(persons.filter(uuid.eq(input_uuid)))
        .set(&person)
        .get_result::<models::people::Person>(&connection)
        .expect(&format!("Unable to find person {}", input_uuid));

      response = helpers::status::Response {
        success: true,
        code: 200,
        data: result.to_json()
      };

      res.set(StatusCode::Ok);

    } else {
      let error = helpers::status::Error {
        error : "A pre-condition of the identifier server-side was not fulfilled".to_string(),
        case: Json::from_str("{}").unwrap()
      };
      response = helpers::status::Response {
        success: false,
        code: 412,
        data: error.to_json()
      };

      res.set(StatusCode::PreconditionFailed);

    }
  } else {
    let case = format!("{:?}", validation_json);
    let error = helpers::status::Error {
      error : "There is one or more validation error".to_string(),
      case: Json::from_str(&case).unwrap()
    };

    response = helpers::status::Response {
      success: false,
      code: 412,
      data: error.to_json()
    };
    res.set(StatusCode::PreconditionFailed);
  }


  res.set(MediaType::Json);
  res.send(response.to_json())
}

fn validate_post (t: &models::people::NewPerson) -> BTreeMap<String, Vec<String>> {
  let mut validations: BTreeMap<String, Vec<String>> = BTreeMap::new();

  validations.insert("email".to_string(), vec![t.email.to_string(), helpers::validator::ValidTypes::Email.to_string()]);
  validations.insert("first_name".to_string(), vec![t.first_name.to_string(), helpers::validator::ValidTypes::AlphaNumberic.to_string()]);
  validations.insert("last_name".to_string(), vec![t.last_name.to_string(), helpers::validator::ValidTypes::AlphaNumberic.to_string()]);

  let result = helpers::validator::validate_map(validations);

  result
}

pub fn post<'a>(req: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
  use lib::schema::persons::dsl::{persons, email};

  let person = try_with!(res, {
    req.json_as::<models::people::NewPerson>().map_err(|e| (StatusCode::BadRequest, e))
  });

  let validation: BTreeMap<String, Vec<String>> = validate_post(&person);
  let validation_json = validation.clone();
  let is_valid = helpers::validator::has_errors(validation);

  let response;

  if is_valid {
    let connection = lib::db::establish_connection();

    let results = persons.filter(email.eq(&person.email))
      .load::<models::people::Person>(&connection)
      .expect("error pulling person matching uuid");

    if results.len() == 1 {
      let error = helpers::status::Error {
        error : "Another person is already associated with that email address.".to_string(),
        case: Json::from_str("{}").unwrap()
      };

      response = helpers::status::Response {
        success: false,
        code: 409,
        data: error.to_json()
      };
      res.set(StatusCode::Conflict);
    } else {
      let result = diesel::insert(&person)
        .into(lib::schema::persons::table)
        .get_result::<models::people::Person>(&connection)
        .expect("create_user_fail");

      response = helpers::status::Response {
        success: true,
        code: 201,
        data: result.to_json()
      };

      res.set(StatusCode::Created);
    }

  } else {
    let case = format!("{:?}", validation_json);
    let error = helpers::status::Error {
      error : "There is one or more validation error".to_string(),
      case: Json::from_str(&case).unwrap()
    };

    response = helpers::status::Response {
      success: false,
      code: 412,
      data: error.to_json()
    };
    res.set(StatusCode::PreconditionFailed);
  }


  res.set(MediaType::Json);
  res.send(response.to_json())
}

pub fn delete<'a>(req: &mut Request, mut res: Response<'a>) -> MiddlewareResult<'a> {
  use lib::schema::persons::dsl::{persons, uuid};
  let validation: BTreeMap<String, Vec<String>> = validate_get_delete(req.param("uuid").unwrap());
  let validation_json = validation.clone();
  let is_valid = helpers::validator::has_errors(validation);

  let response;

  if is_valid {
      
    let connection = lib::db::establish_connection();

    let input_uuid = Uuid::parse_str(req.param("uuid").unwrap()).unwrap();
    let result = diesel::delete(persons.filter( uuid.eq(input_uuid)))
      .execute(&connection)
      .expect("Error deleting person");

    if result == 1 {
      let message = "The resource has been destroyed".to_string();
      response = helpers::status::Response {
        success: true,
        code: 410,
        data: message.to_json()
      };
    } else {
      let error = helpers::status::Error {
        error : "The resource looks to have already been destroyed".to_string(),
        case: Json::from_str("{}").unwrap()
      };

      response = helpers::status::Response {
        success: false,
        code: 410,
        data: error.to_json()
      }
    }
    res.set(StatusCode::Gone);
  } else {
    let case = format!("{:?}", validation_json);
    let error = helpers::status::Error {
      error : "There is one or more validation error".to_string(),
      case: Json::from_str(&case).unwrap()
    };

    response = helpers::status::Response {
      success: false,
      code: 412,
      data: error.to_json()
    };
    res.set(StatusCode::PreconditionFailed);
  }

  res.set(MediaType::Json);
  res.send(response.to_json())
}
