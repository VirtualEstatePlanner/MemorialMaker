use crate::structs::IContactPerson;
use crate::structs::ILivestreamInfo;
use crate::structs::ILocation;
// use crate::structs::IServiceAgendaItem;
use crate::structs::UServiceType;

//use chrono::DateTime;
//use chrono::Utc;

use uuid::Uuid;

pub struct IService {
  key: Uuid,
  additional_info: String,
  contact: IContactPerson::IContactPerson,
  livestream: ILivestreamInfo::ILivestreamInfo,
  location: ILocation::ILocation,
  // programme: Vec<IServiceAgendaItem::IServiceAgendaItem>,
  service_type: String,
  // time: DateTime<Utc>,
}

pub fn new() -> IService {
  return IService {
    key: Uuid::new_v4(),
    additional_info: "additional-service-info-string".to_string(),
    contact: IContactPerson::new(),
    livestream: ILivestreamInfo::new(),
    location: ILocation::new(),
    // programme: Vec::<IServiceAgendaItem>::new(),
    service_type: UServiceType::BURIAL.to_string(),
    // time: DateTime::<Utc>::from_utc(Utc::now(), Utc),
  }
}
