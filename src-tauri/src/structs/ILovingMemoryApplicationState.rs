mod IAppPreferences;
mod ICharity;
mod IContactPerson;
mod IMemorializedPerson;
mod IService;
mod UContentItem;

pub fn new() {
    struct IAppState {
    }
}

pub struct ILovingMemoryApplicationState {
  preferences: IAppPreferences,
  memorialized: IMemorializedPerson,
  burial: IService,
  cremation: IService,
  funeral: IService,
  scattering: IService,
  charities: Vector<ICharity>,
  contentItems: Vector<UContentItem>
}
