pub struct ITimeZone {
  name: &str,
  abbreviation: &str,
  offset: i16,
  isDaylightSavingsTime: &bool,
  description: &str,
  utc: Vector<&str>
}
