
use std::str::FromStr;

pub struct Area {
    pub id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub quests: Vec<Quest>, // TODO: does this really want to be embedded, or just trackable via uuid?
}
impl std::fmt::Debug for Area {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Area {{\n\tUUID: {}\n\tName: {}\n\tDescription: {}\n\tQuest Count: {}\n}}",
            self.id, self.name, self.description, self.quests.len(),
        )
    }
}
impl Area {
    pub fn new(name: &str, description: &str, quests: &[Quest]) -> Self {
        let area_id = uuid::Uuid::new_v4();
        let mut quests_with_area_ids: Vec<Quest> = vec![];
        for quest in quests {
            let mut quest_with_area_id = quest.clone();
            quest_with_area_id.area_id = Some(area_id);
            quests_with_area_ids.push(quest_with_area_id);
        } // TODO: this is awful, and I tried to do it better, but failed, try again later
        Area {
            id: area_id,
            name: name.to_owned(),
            description: description.to_owned(),
            quests: quests_with_area_ids,
        }
    }
}

#[derive(Copy, Clone, Debug, strum_macros::EnumCount, strum_macros::EnumIter, strum_macros::EnumString)]
pub enum QuestType {
    Genocide, // slay X number of ENEMYs
    Loot, // retrieve X number of ITEMs from ENEMYs
    Boss, // slay BOSS
    Fetch, // collect X number of ITEMs
    Talk, // talk to NPC
}

#[derive(Clone)]
pub struct Quest {
    pub id: uuid::Uuid,
    pub area_id: Option<uuid::Uuid>,
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
        let specifics = match self.the_type {
            QuestType::Genocide => {
                format!(
                    "\n\tNumber: {}\n\tEnemy: {}",
                    self.number.as_ref().unwrap(), self.enemy.as_ref().unwrap(),
                )
            },
            QuestType::Loot => {
                format!(
                    "\n\tNumber: {}\n\tItem: {}\n\tEnemy: {}",
                    self.number.as_ref().unwrap(), self.item.as_ref().unwrap(), self.enemy.as_ref().unwrap(),
                )
            },
            QuestType::Boss => {
                format!("\n\tBoss: {}", self.boss.as_ref().unwrap())
            },
            QuestType::Fetch => {
                format!(
                    "\n\tNumber: {}\n\tItem: {}",
                    self.number.as_ref().unwrap(), self.item.as_ref().unwrap())
            },
            QuestType::Talk => {
                format!("\n\tNPC: {}", self.npc.as_ref().unwrap())
            },
        };
        write!(
            f,
            "Quest {{\n\tUUID: {}\n\tArea UUID: {:?}\n\tGiver: {}\n\tDescription: {}\n\tType: {:?}{specifics}\n}}",
            self.id, self.area_id, self.giver, self.description, self.the_type,
        )
    }
}
impl Quest {
    pub fn new_boss(giver: &str, description: &str, boss: &str) -> Self {
        Quest {
            id: uuid::Uuid::new_v4(),
            area_id: None,
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
            id: uuid::Uuid::new_v4(),
            area_id: None,
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
            id: uuid::Uuid::new_v4(),
            area_id: None,
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
            id: uuid::Uuid::new_v4(),
            area_id: None,
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
            id: uuid::Uuid::new_v4(),
            area_id: None,
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

#[derive(serde::Deserialize)]
pub struct UuidRequest {
    pub uuid: String,
}

#[derive(serde::Serialize)]
pub struct StringArea { // Used for json and sqlite
    pub uuid: String,
    pub name: String,
    pub description: String,
}
impl StringArea {
    pub fn from_area(area: &Area) -> Self {
        Self {
            uuid: area.id.hyphenated().to_string(),
            name: area.name.to_owned(),
            description: area.description.to_owned(),
        }
    }
    pub fn to_area(self) -> Area {
        Area {
            id: uuid::Uuid::from_str(&self.uuid).unwrap(),
            name: self.name,
            description: self.description,
            quests: vec![], // TODO: shouldn't exist, or call some get_quest fn
        }
    }
}

#[derive(serde::Serialize)]
pub struct StringQuest { // Used for json and sqlite
    pub uuid: String,
    pub area_id: String,
    pub the_type: String,
    pub giver: String,
    pub description: String,
    pub number: String,
    pub enemy: String,
    pub item: String,
    pub boss: String,
    pub npc: String,
}
impl StringQuest {
    pub fn from_quest(quest: &Quest) -> Self {
        Self {
            uuid: quest.id.hyphenated().to_string(),
            area_id: {if let Some(id) = quest.area_id { id.hyphenated().to_string() } else { String::new() }},
            the_type: format!("{:?}", quest.the_type),
            giver: quest.giver.to_owned(),
            description: quest.description.to_owned(),
            number: {if let Some(number) = quest.number { number.to_string() } else { String::new() }},
            enemy: quest.enemy.to_owned().unwrap_or(String::new()),
            item: quest.item.to_owned().unwrap_or(String::new()),
            boss: quest.boss.to_owned().unwrap_or(String::new()),
            npc: quest.npc.to_owned().unwrap_or(String::new()),
        }
    } 
    pub fn to_quest(self) -> Quest {
        Quest {
            id: uuid::Uuid::from_str(&self.uuid).unwrap(),
            area_id: if self.area_id.is_empty() { None } else { Some(uuid::Uuid::from_str(&self.area_id).unwrap()) },
            the_type: QuestType::from_str(&self.the_type).unwrap(),
            giver: self.giver,
            description: self.description,
            number: if self.number.is_empty() { None } else { Some(self.number.parse::<u8>().unwrap()) },
            enemy: if self.enemy.is_empty() { None } else { Some(self.enemy) },
            item: if self.item.is_empty() { None } else { Some(self.item) },
            boss: if self.boss.is_empty() { None } else { Some(self.boss) },
            npc: if self.npc.is_empty() { None } else { Some(self.npc) },
        }
    }
}