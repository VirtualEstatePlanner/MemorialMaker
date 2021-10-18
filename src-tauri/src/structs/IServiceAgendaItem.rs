use uuid::Uuid;

pub struct IServiceAgendaItem {
  key: Uuid,
  description: String,
  value: String,
}

pub fn new() -> IServiceAgendaItem {
  return IServiceAgendaItem {
    key: Uuid::new_v4(),
    description: "agenda-item-description".to_string(),
    value: "agenda-item-value".to_string(),
  };
}
