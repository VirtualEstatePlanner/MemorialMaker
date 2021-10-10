pub struct ILivestreamCredentials {
  username: &str,
  password: &str
}

pub fn new() -> ILivestreamCredentials {
  ILivestreamCredentials {
    username: "livestream-username",
    password: "livestream-password"
  }
}