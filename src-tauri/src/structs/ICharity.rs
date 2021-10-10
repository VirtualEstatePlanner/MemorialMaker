use url::URL;

pub struct ICharity {
  name: &str,
  issue: &str,
  url: URL,
}

pub fn new() -> ICharity {
  ICharity {
    name: "charity-name",
    issue: "charity-issue",
    url: URL::parse("default-url-location.com").unwrap(),
  }
}
