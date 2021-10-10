pub struct INation {
  alpha2: &str,
  alpha3: &str,
  name: &str
}

pub fn new() -> INation {
  INation {
    alpha2: "US",
    alpha3: "USA",
    name: "United States of America"
  }
}