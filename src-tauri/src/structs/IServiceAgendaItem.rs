use uuid::Uuid;

pub struct IAgendaItem {
  key: &str,
  description: &str,
  value: &str
}

pub fn new() -> IAgendaItem {
  IAgendaItem {
    key: Uuid::new_v5(),
    description = "agenda-item-description",
    value = "agenda-item-value",
  };
}
