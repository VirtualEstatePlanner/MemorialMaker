pub struct ITimeZone {
  name: String,
  abbreviation: String,
  offset: i16,
  isDaylightSavingsTime: bool,
  description: String,
  utc: Vec<String>,
}

pub fn new() -> ITimeZone {
  ITimeZone {
    name: "time-zone-name".to_string(),
    abbreviation: "TZN".to_string(),
    offset: 0,
    isDaylightSavingsTime: false,
    description: "default-time-zone-description".to_string(),
    utc: Vec::new(),
  }
}
