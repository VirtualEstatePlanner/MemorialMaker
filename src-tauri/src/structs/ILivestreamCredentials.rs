pub struct ILivestreamCredentials {
  username: String,
  password: String,
}

pub fn new() -> ILivestreamCredentials {
  ILivestreamCredentials {
    username: "livestream-username".to_string(),
    password: "livestream-password".to_string(),
  }
}
