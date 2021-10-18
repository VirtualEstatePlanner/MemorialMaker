pub struct IAppWidePreferences {
  display_only_middle_initial: bool,
}

impl IAppWidePreferences {
  pub fn new(display_only_middle_initial: bool) -> Self {
    Self {
      display_only_middle_initial,
    }
  }
}

pub fn new() -> IAppWidePreferences {
  return IAppWidePreferences {
    display_only_middle_initial: true,
  };
}
