pub struct Area {
    pub name: String,
    pub description: String,
    pub quests: Vec<Quest>,
}
// TODO: impl std::fmt::Debug

#[derive(Debug)]
pub enum QuestType {
    Genocide,
}

pub struct Quest {
    pub the_type: QuestType,
    pub giver: String,
    pub description: String,
    pub enemy_name: Option<String>,
}
impl std::fmt::Debug for Quest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Quest {{\n\tType: {:?}\n\tGiver: {}\n\tDescription: {}\n\tEnemy: {:?}\n}}",
            self.the_type, self.giver, self.description, self.enemy_name,
        )
    }
}