pub struct Area {
    pub name: String,
    pub description: String,
    pub quests: Vec<Quest>,
}
// TODO: impl std::fmt::Debug

#[derive(Debug, strum_macros::EnumCount, strum_macros::EnumIter)]
pub enum QuestType {
    Genocide, // slay X number of ENEMYs
    Loot, // retrieve X number of ITEMs from ENEMYs
    Boss, // slay BOSS
    Fetch, // collect X number of ITEMs
    Talk, // talk to NPC
}

pub struct Quest {
    pub the_type: QuestType,
    pub giver: String,
    pub description: String,
    pub number: Option<u8>,
    pub enemy: Option<String>,
    pub item: Option<String>,
    pub boss: Option<String>,
    pub npc: Option<String>,
}
impl std::fmt::Debug for Quest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Quest {{\n\tType: {:?}\n\tGiver: {}\n\tDescription: {}\n}}",
            self.the_type, self.giver, self.description,
        ) // TODO: match for what option(s) to print
    }
}
impl Quest {
    pub fn new_boss(giver: &str, description: &str, boss: &str) -> Self {
        Quest {
            the_type: QuestType::Boss,
            giver: giver.to_owned(),
            description: description.to_owned(),
            number: None,
            enemy: None,
            item: None,
            boss: Some(boss.to_owned()),
            npc: None,
        }
    }
    pub fn new_fetch(giver: &str, description: &str, number: u8, item: &str) -> Self {
        Quest {
            the_type: QuestType::Fetch,
            giver: giver.to_owned(),
            description: description.to_owned(),
            number: Some(number),
            enemy: None,
            item: Some(item.to_owned()),
            boss: None,
            npc: None,
        }
    }
    pub fn new_genocide(giver: &str, description: &str, number: u8, enemy: &str) -> Self {
        Quest {
            the_type: QuestType::Genocide,
            giver: giver.to_owned(),
            description: description.to_owned(),
            number: Some(number),
            enemy: Some(enemy.to_owned()),
            item: None,
            boss: None,
            npc: None,
        }
    }
    pub fn new_loot(giver: &str, description: &str, number: u8, item: &str, enemy: &str) -> Self {
        Quest {
            the_type: QuestType::Loot,
            giver: giver.to_owned(),
            description: description.to_owned(),
            number: Some(number),
            enemy: Some(enemy.to_owned()),
            item: Some(item.to_owned()),
            boss: None,
            npc: None,
        }
    }
    pub fn new_talk(giver: &str, description: &str, npc: &str) -> Self {
        Quest {
            the_type: QuestType::Genocide,
            giver: giver.to_owned(),
            description: description.to_owned(),
            number: None,
            enemy: None,
            item: None,
            boss: None,
            npc: Some(npc.to_owned()),
        }
    }
}