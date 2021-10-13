pub struct IAppWidePreferences {
  display_only_middle_initial: bool,
}

pub fn new() -> IAppWidePreferences {
  IAppWidePreferences {
    display_only_middle_initial: true,
  }
}
