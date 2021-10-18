use crate::structs::IAppWidePreferences;
use crate::structs::IServicePreferences;

pub struct IAppPreferences {
  app_wide: IAppWidePreferences::IAppWidePreferences,
  burial_preferences: IServicePreferences::IServicePreferences,
  cremation_preferences: IServicePreferences::IServicePreferences,
  funeral_preferences: IServicePreferences::IServicePreferences,
  scattering_preferences: IServicePreferences::IServicePreferences,
}

pub fn new() -> IAppPreferences {
  return IAppPreferences {
    app_wide: IAppWidePreferences::new(),
    burial_preferences: IServicePreferences::new(),
    cremation_preferences: IServicePreferences::new(),
    funeral_preferences: IServicePreferences::new(),
    scattering_preferences: IServicePreferences::new(),
  };
}
