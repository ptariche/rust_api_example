use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};


#[derive(RustcDecodable, RustcEncodable)]
pub struct Person {
    pub first_name: String,
    pub last_name:  String,
}

impl ToJson for Person {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();
        map.insert("first_name".to_string(), self.first_name.to_json());
        map.insert("last_name".to_string(), self.last_name.to_json());
        Json::Object(map)
    }
}
