mod INation;
mod ITimeZone;

use url::URL;

pub struct ILocation {
  location_name: &str,
  street_number: &str,
  street_name: &str,
  city_or_town: &str,
  state_or_province: &str,
  country: INation,
  postal_code: &str,
  map_url: url::URL,
  time_zone: ITimeZone
}

pub fn new() -> ILocation {
  ILocation {
    location_name: "default-location-name",
    street_number: "default-street-number",
    street_name: "default-street-name",
    city_or_town: "default-city-or-town",
    state_or_province: "default-state-or-province",
    country: INation::new(),
    postal_code: "default-postal-code",
    map_url: URL::new(),
    time_zone: ITimeZone::new()
  }
}