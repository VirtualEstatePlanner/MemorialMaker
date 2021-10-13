use crate::structs::IAppPreferences;
use crate::structs::ICharity;
use crate::structs::IMemorializedPerson;
use crate::structs::IService;
use crate::structs::UContentItem;

pub struct ILovingMemoryApplicationState {
  preferences: IAppPreferences::IAppPreferences,
  memorialized: IMemorializedPerson::IMemorializedPerson,
  burial: IService::IService,
  cremation: IService::IService,
  funeral: IService::IService,
  scattering: IService::IService,
  charities: Vec<ICharity::ICharity>,
  content_items: Vec<UContentItem::UContentItem>,
}

pub fn new() -> ILovingMemoryApplicationState {
  ILovingMemoryApplicationState {
    preferences: IAppPreferences::new(),
    memorialized: IMemorializedPerson::new(),
    burial: IService::new(),
    cremation: IService::new(),
    funeral: IService::new(),
    scattering: IService::new(),
    charities: Vec::<ICharity::ICharity>::new(),
    content_items: Vec::<UContentItem::UContentItem>::new(),
  }
}
