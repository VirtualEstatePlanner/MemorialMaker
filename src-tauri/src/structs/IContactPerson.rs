mod IName;
mod ITimeZone;

pub struct IContactPerson {
  name: IName,
  phone_number: &str,
  email: &str,
  time_zone: ITimeZone,
}

pub fn new() -> IContactPerson {
  IContactPerson {
    name: IName::new(),
    phone_number: "contact-phone-number",
    email: "usernmame@domain.tld",
    time_zone: ITimeZone::new(),
  }
}
