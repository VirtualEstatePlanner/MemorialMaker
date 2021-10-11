mod IAppWidePreferences;
mod IServicePrefernces;

pub struct IAppPreferences {
  app_wide: IAppWidePreferences,
  burial_preferences: IServicePreferences,
  cremation_preferences: IServicePreferences,
  funeral_preferences: IServicePreferences,
  scattering_preferences: IServicePreferences,
}

pub fn new() -> IAppPreferences {
  IAppPreferences {
    app_wide: IAppWidePreferences::new(),
    burial_preferences: IServicePreferences::new(),
    cremation_preferences: IServicePreferences::new(),
    funeral_preferences: IServicePreferences::new(),
    scattering_preferences: IServicePreferences::new(),
  }
}