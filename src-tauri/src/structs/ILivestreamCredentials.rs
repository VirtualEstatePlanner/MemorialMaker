pub struct ILivestreamCredentials {
  username: String,
  password: String,
}

pub fn new() -> ILivestreamCredentials {
  return ILivestreamCredentials {
    username: "livestream-username".to_string(),
    password: "livestream-password".to_string(),
  }
}
