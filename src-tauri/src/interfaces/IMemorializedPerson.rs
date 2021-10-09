// needs to extend person struct

struct IName<IMemorializedPerson> {
    pub dateOfBirth: chrono::DateTime<chrono::Utc>,
    pub dateOfDeath: chrono::DateTime<chrono::Utc>
}