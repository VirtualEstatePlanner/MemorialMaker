use crate::structs::UServiceType;

pub struct IServicePreferences {
  service_type: String,
  are_having: bool,
  are_livestreaming: bool,
  livestream_is_public: bool,
}

pub fn new() -> IServicePreferences {
  return IServicePreferences {
    service_type: UServiceType::BURIAL.to_string(),
    are_having: false,
    are_livestreaming: false,
    livestream_is_public: false,
  }
}
