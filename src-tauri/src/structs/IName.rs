use crate::structs::UNamePrefix;
use crate::structs::UNameSuffix;

pub struct IName {
  pub first: String,
  pub middle: String,
  pub last: String,
  pub prefix: String,
  pub suffix: String,
}

pub fn new() -> IName {
  IName {
    first: "first-name".to_string(),
    middle: "middle-name".to_string(),
    last: "last-name".to_string(),
    prefix: UNamePrefix::NONE,
    suffix: UNameSuffix::NONE,
  }
}
