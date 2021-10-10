mod IName;

pub struct IMemorializedPerson {
  person: IName,
  dateOfBirth: chrono::DateTime<chrono::Utc>,
  dateOfDeath: chrono::DateTime<chrono::Utc>
}

pub fn new() -> IMemorializedPerson {
  IMemorializedPerson {
    person: IName::new(),
    dateOfBirth: chrono::DateTime::<chrono::Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(0, 0), chrono::Utc),
    dateOfDeath: chrono::DateTime::<chrono::Utc>::from_utc(chrono::NaiveDateTime::from_timestamp(0, 0), chrono::Utc)
  }
}