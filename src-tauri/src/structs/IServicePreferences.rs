mod UServiceType;

pub struct IServicePreferences {
  service_type: UServiceType,
  are_having: &bool,
  are_livestreaming: &bool,
  livestream_is_public: &bool
}

pub fn new() -> IServicePreferences {
  IServicePreferences {
    service_type: UServiceType::new(),
    are_having: &false,
    are_livestreaming: &false,
    livestream_is_public: &false
  }
}
