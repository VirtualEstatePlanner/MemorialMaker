pub struct IAgendaItem {
    key: &str,
    description: &str,
    value: &str
}

pub fn createServiceAgendaItem(description: &str, value: &str) -> IAgendaItem {
    const key: &str = std::rand::random::<u64>();
    const description: &str = description;
    const value: &str = value;

    return {key; description; value;}
}