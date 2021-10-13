pub struct INation {
  alpha2: String,
  alpha3: String,
  name: String,
}

pub fn new() -> INation {
  INation {
    alpha2: "US".to_string(),
    alpha3: "USA".to_string(),
    name: "United States of America".to_string(),
  }
}
