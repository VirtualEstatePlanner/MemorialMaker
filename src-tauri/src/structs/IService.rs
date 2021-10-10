pub struct IService {
  key: &str,
  additionalInfo: &str,
  contact: IContactPerson,
  livestream: ILivestreamInfo,
  location: ILocation,
  serviceType: TServiceType,
  programme: Vector<IServiceAgendaItem>,
  time: date::DateTime<Utc>,
}
