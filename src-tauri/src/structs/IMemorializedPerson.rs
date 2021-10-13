use crate::structs::IName;
use chrono::{DateTime, NaiveDateTime, Utc};

pub struct IMemorializedPerson {
  person: IName::IName,
  dateOfBirth: DateTime<Utc>,
  dateOfDeath: DateTime<Utc>,
}

pub fn new() -> IMemorializedPerson {
  IMemorializedPerson {
    person: IName::new(),
    dateOfBirth: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
    dateOfDeath: DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
  }
}
