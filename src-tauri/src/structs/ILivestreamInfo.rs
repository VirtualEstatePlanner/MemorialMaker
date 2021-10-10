mod ILivestreamCredentials;

use url::URL;

pub struct ILivestreamInfo {
  requires_login: bool,
  url: url::URL,
  credentials: ILivestreamCredentials::new(),
}

pub fn new() -> ILivestreamInfo {
  ILivestreamInfo {
    requires_login: false,
    url: URL::parse("default-url-location.com"),
    credentials: ILivestreamCredentials::new()
  }
}