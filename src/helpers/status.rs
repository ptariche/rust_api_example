use std::collections::BTreeMap;
use rustc_serialize::json::{Json, ToJson};

pub struct Response {
    pub success: bool,
    pub code: u8,
    pub data: Json
}

impl ToJson for Response {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();

        map.insert("success".to_string(), self.success.to_json());
        map.insert("code".to_string(), self.code.to_json());
        map.insert("data".to_string(), self.data.to_json());

        Json::Object(map)
    }
}

pub struct Error {
    pub error: String,
}


impl ToJson for Error {
    fn to_json(&self) -> Json {
        let mut map = BTreeMap::new();

        map.insert("error".to_string(), self.error.to_json());

        Json::Object(map)
    }
}