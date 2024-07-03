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