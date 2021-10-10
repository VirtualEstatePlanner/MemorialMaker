pub struct IName {
  pub first: &str,
  pub middle: &str,
  pub last: &str,
  pub prefix: UNamePrefix,
  pub suffix: UNameSuffix,
}

pub fn new() -> IName {
  IName {
    first: "first-name",
    middle: "middle-name",
    last: "last-name",
    prefix: UNamePrefix::NONE,
    suffix: UNameSuffix::NONE,
  }
}