pub struct IEmailAddress {
  username: String,
  domain: String,
  tld: String,
}

pub fn new() -> IEmailAddress {
  IEmailAddress {
    username: "user".to_string(),
    domain: "domain".to_string(),
    tld: "tld".to_string(),
  }
}
