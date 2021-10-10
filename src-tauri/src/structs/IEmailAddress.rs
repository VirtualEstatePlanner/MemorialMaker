pub struct IEmailAddress {
  pub username: &str,
  pub domain: &str,
  pub tld: &str,
}

pub fn new() -> IEmailAddress {
  IEmailAddress {
    username: "user",
    domain: "domain",
    tld: "tld",
  }
}