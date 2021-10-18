use crate::structs::INation;
use crate::structs::ITimeZone;

use url::Url;

pub struct ILocation {
  location_name: String,
  street_number: String,
  street_name: String,
  city_or_town: String,
  state_or_province: String,
  country: INation::INation,
  postal_code: String,
  map_url: Url,
  time_zone: ITimeZone::ITimeZone,
}

pub fn new() -> ILocation {
  return ILocation {
    location_name: "default-location-name".to_string(),
    street_number: "default-street-number".to_string(),
    street_name: "default-street-name".to_string(),
    city_or_town: "default-city-or-town".to_string(),
    state_or_province: "default-state-or-province".to_string(),
    country: INation::new(),
    postal_code: "default-postal-code".to_string(),
    map_url: Url::parse("default-location-url.com").unwrap(),
    time_zone: ITimeZone::new(),
  }
}
