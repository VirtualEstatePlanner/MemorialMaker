mod IAppPreferences;
mod ICharity;
mod IContactPerson;
mod IMemorializedPerson;
mod IService;
mod UContentItem;

pub struct ILovingMemoryApplicationState {
  preferences: IAppPreferences,
  memorialized: IMemorializedPerson,
  burial: IService,
  cremation: IService,
  funeral: IService,
  scattering: IService,
  charities: Vec<ICharity>,
  content_items: Vec<UContentItem>
}

pub fn new() -> ILovingMemoryApplicationState {
  ILovingMemoryApplicationState {
    preferences: IAppPreferences::new(),
    memorialized: IMemorializedPerson::new(),
    burial: IService::new(),
    cremation: IService::new(),
    funeral: IService::new(),
    scattering: IService::new(),
    charities: Vec::new(),
    content_items: Vec::new()
  }
}
