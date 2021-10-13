use crate::structs::IEmailAddress;
use crate::structs::IName;
use crate::structs::ITimeZone;

pub struct IContactPerson {
  name: IName::IName,
  phone_number: String,
  email: IEmailAddress::IEmailAddress,
  time_zone: ITimeZone::ITimeZone,
}

pub fn new() -> IContactPerson {
  IContactPerson {
    name: IName::new(),
    phone_number: "contact-phone-number".to_string(),
    email: IEmailAddress::new(),
    time_zone: ITimeZone::new(),
  }
}
