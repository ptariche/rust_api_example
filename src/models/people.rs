use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};
use chrono::NaiveDateTime;
use uuid::Uuid;

use lib::schema::persons;

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Queryable, Identifiable, Associations, AsChangeset)]
pub struct Person {
  pub id: i32,
  pub email: String,
  pub first_name: String,
  pub last_name: String,
  pub uuid: Uuid,
  pub created_at: NaiveDateTime,
  pub updated_at: NaiveDateTime,
}

impl ToJson for Person {
  fn to_json(&self) -> Json {
    let mut map = BTreeMap::new();

    map.insert("email".to_string(), self.email.to_json());
    map.insert("first_name".to_string(), self.first_name.to_json());
    map.insert("last_name".to_string(), self.last_name.to_json());
    map.insert("uuid".to_string(), self.uuid.hyphenated().to_string().to_json());

    Json::Object(map)
  }
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Insertable)]
#[table_name="persons"]
pub struct NewPerson {
  pub email: String,
  pub first_name: String,
  pub last_name: String,
}

impl ToJson for NewPerson {
  fn to_json(&self) -> Json {
    let mut map = BTreeMap::new();

    map.insert("email".to_string(), self.email.to_json());
    map.insert("first_name".to_string(), self.first_name.to_json());
    map.insert("last_name".to_string(), self.last_name.to_json());

    Json::Object(map)
  }
}

#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Insertable, AsChangeset)]
#[table_name="persons"]
pub struct UpdatePerson {
  pub first_name: String,
  pub last_name: String,
}

impl ToJson for UpdatePerson {
  fn to_json(&self) -> Json {
    let mut map = BTreeMap::new();

    map.insert("first_name".to_string(), self.first_name.to_json());
    map.insert("last_name".to_string(), self.last_name.to_json());

    Json::Object(map)
  }
}
