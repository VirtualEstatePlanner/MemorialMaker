use crate::structs::IName;
use chrono::{DateTime, NaiveDateTime, Utc};

pub struct IMemorializedPerson {
  person: IName::IName,
  date_of_birth: DateTime<Utc>,
  date_of_death: DateTime<Utc>,
}

pub fn new() -> IMemorializedPerson {
  return IMemorializedPerson {
    person: IName::new(),
    date_of_birth: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
    date_of_death: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
  }
}
