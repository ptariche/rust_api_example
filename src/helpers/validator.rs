use std::collections::BTreeMap;
use regex::Regex;
use std::fmt;

#[allow(dead_code)]
#[derive(Debug, Copy, Clone)]
pub enum ValidTypes {
  Email,
  USPhone,
  USZipCode,
  AlphaNumberic,
  Uuid,
  None
}

impl fmt::Display for ValidTypes {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

fn string_to_valid_types(s: &str) -> ValidTypes {
  match s {
    "Email" => ValidTypes::Email,
    "USPhone" => ValidTypes::USPhone,
    "USZipCode" => ValidTypes::USZipCode,
    "AlphaNumberic" => ValidTypes::AlphaNumberic,
    "Uuid" => ValidTypes::Uuid,
     _ => ValidTypes::None
  }
}

fn error (value_type: ValidTypes, valid: bool) -> Vec<String> {
  let message;

  if valid {
    message = "is valid".to_string();
  } else {
    message = match value_type {
      ValidTypes::Email => "A valid Email is required".to_string(),
      ValidTypes::USPhone => "A valid US Phone number is required".to_string(),
      ValidTypes::USZipCode => "A valid US Zip code is required".to_string(),
      ValidTypes::AlphaNumberic => "Only alpha numeric values are allowed".to_string(),
      ValidTypes::Uuid => "A valid uuid is required".to_string(),
      ValidTypes::None => "An unknown error has occured".to_string(),
    };
  }
  vec![valid.to_string(), message]
}

pub fn has_errors (result: BTreeMap<String, Vec<String>>) -> bool {
  let mut valid = true;
  for (_key, value) in result.iter() {
    if value[0].to_string() == "false".to_string() {
      valid = false;
    }
  }

  valid
}

pub fn validate_map (plural: BTreeMap<String, Vec<String>>) -> BTreeMap<String, Vec<String>> {
  let mut report: BTreeMap<String, Vec<String>> = BTreeMap::new();
  for (key, value) in plural.iter() {
    let valid_type = string_to_valid_types(&value[1]);
    let valide = validate(&value[0].to_string(), valid_type);
    report.insert(key.to_string(), error(valid_type, valide));
  }

  report
}

pub fn validate (value: &String, value_type: ValidTypes) -> bool {
  if value.is_empty() {
    return false;
  } else {
    let re = match value_type {
      ValidTypes::Email => Regex::new(r".+@.+\..+").unwrap(),
      ValidTypes::USPhone => Regex::new("[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap(),
      ValidTypes::USZipCode => Regex::new(r"^[0-9]{5}(?:-[0-9]{4})?$").unwrap(),
      ValidTypes::AlphaNumberic => Regex::new(r"^[A-Za-z0-9]+$").unwrap(),
      ValidTypes::Uuid => Regex::new("[a-f0-9]{8}-?[a-f0-9]{4}-?[1-5][a-f0-9]{3}-?[89ab][a-f0-9]{3}-?[a-f0-9]{12}").unwrap(),
      ValidTypes::None => Regex::new("[a-f0-9]{8}-?[a-f0-9]{4}-?[1-5][a-f0-9]{3}-?[89ab][a-f0-9]{3}-?[a-f0-9]{12}").unwrap(),
    };

    re.is_match(&value)
  }

} 