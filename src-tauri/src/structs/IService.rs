mod IContactPerson;
mod ILivestreamCredentials;
mod ILivestreamInfo;
mod ILocation;
mod IServiceAgendaItem;
mod UServiceType;

use date::DateTime;
use uuid::Uuid;

pub struct IService {
  key: Uuid::new_v5(),
  additional_info: &str,
  contact: IContactPerson,
  livestream: ILivestreamInfo,
  location: ILocation,
  service_type: UServiceType,
  programme: Vec<IServiceAgendaItem>,
  time: DateTime<Utc>,
}

pub fn new() -> IService {
  IService {
    key: Uuid::new_v5(),
    additional_info: "additional-service-info-string",
    contact: IContactPerson::new(),
    livestream: ILivestreamInfo::new(),
    location: ILocation::new(),
    service_type: UServiceType::new(),
    programme: Vec::new(),
    time: DateTime::<Utc>::now(),
  }
}