mod IAppWidePreferences;
mod IServicePrefernces;

pub struct IAppPreferences {
  app_wide: IAppWidePreferences,
  service_preferences: Vec<IServicePreferences>,
}

pub fn new() -> IAppPreferences {
  IAppPreferences {
    app_wide: IAppWidePreferences::new(),
    service_preferences: Vec<IServicePreferences>::new(),
  }
}