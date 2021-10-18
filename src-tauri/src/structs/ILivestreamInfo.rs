use crate::structs::ILivestreamCredentials;

use url::Url;

pub struct ILivestreamInfo {
  requires_login: bool,
  url: url::Url,
  credentials: ILivestreamCredentials::ILivestreamCredentials,
}

pub fn new() -> ILivestreamInfo {
  return ILivestreamInfo {
    requires_login: false,
    url: Url::parse("default-url-location.com").unwrap(),
    credentials: ILivestreamCredentials::new(),
  };
}
