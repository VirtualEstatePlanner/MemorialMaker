pub struct ITimeZone {
  name: &str,
  abbreviation: &str,
  offset: i16,
  isDaylightSavingsTime: &bool,
  description: &str,
  utc: Vec<&str>
}

pub fn new() -> ITimeZone {
  ITimeZone {
    name: "time-zone-name",
    abbreviation: "TZN",
    offset: 0,
    isDaylightSavingsTime: &false,
    description: "default-time-zone-description",
    utc: Vec::new()
  }
}