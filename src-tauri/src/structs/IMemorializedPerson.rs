struct IMemorializedPerson {
  person: IName,
  dateOfBirth: chrono::DateTime<chrono::Utc>,
  dateOfDeath: chrono::DateTime<chrono::Utc>
}