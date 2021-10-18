use url::Url;

pub struct ICharity {
  name: String,
  issue: String,
  url: Url,
}

pub fn new() -> ICharity {
  return ICharity {
    name: "charity-name".to_string(),
    issue: "charity-issue".to_string(),
    url: Url::parse("default-url-location.com").unwrap(),
  };
}
