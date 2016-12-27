use regex::Regex;

#[allow(dead_code)]
pub enum ValidTypes {
  Email,
  USPhone,
  USZipCode,
  AlphaNumberic,
}

pub fn validate (value: &String, value_type: ValidTypes) -> bool {
  if value.is_empty() {
    return false;
  } else {
    let re = match value_type {
      ValidTypes::Email => Regex::new(r".+@.+\..+").unwrap(),
      ValidTypes::USPhone => Regex::new("[0-9]{3}-[0-9]{3}-[0-9]{4}").unwrap(),
      ValidTypes::USZipCode => Regex::new("^[0-9]{5}(?:-[0-9]{4})?$").unwrap(),
      ValidTypes::AlphaNumberic => Regex::new("^[A-Za-z0-9]+$").unwrap(),
    };

    re.is_match(&value)
  }

} 