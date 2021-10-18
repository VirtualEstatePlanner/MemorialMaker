pub struct ITimeZone {
  name: String,
  abbreviation: String,
  offset: i16,
  is_daylight_savings_time: bool,
  description: String,
  utc: Vec<String>,
}

pub fn new() -> ITimeZone {
  return ITimeZone {
    name: "time-zone-name".to_string(),
    abbreviation: "TZN".to_string(),
    offset: 0,
    is_daylight_savings_time: false,
    description: "default-time-zone-description".to_string(),
    utc: Vec::new(),
  };
}
