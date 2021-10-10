mod INation;

pub struct ILocation {
  locationName: &str,
  streetnumber: &str,
  streetName: &str,
  cityOrProvince: &str,
  state: &str,
  country: INation,
  postalCode: &str,
  mapURL: url::URL,
  timeZone: ITimeZone
}
