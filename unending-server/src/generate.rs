use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;

pub fn create_area_with_quests(
    client: &reqwest::blocking::Client,
    quest_count: usize
) -> (crate::data::Area, Vec<crate::data::Quest>) {
    let mut area = create_area(client);
    let mut quests: Vec<crate::data::Quest> = vec![];
    for _ in 0..quest_count {
        quests.push(create_quest(client, &area));
    }
    let quest_ids: Vec<uuid::Uuid> = quests
        .iter()
        .map(|q| q.id)
        .collect();
    area.quest_ids = quest_ids;
    (area, quests)
}

fn create_area(client: &reqwest::blocking::Client) -> crate::data::Area {
    let area_name = crate::llm::chat(
        client,
        &crate::template::area_name(),
    );
    let area_desc = crate::llm::chat(
        client,
        &crate::template::area_description(&area_name),
    );
    crate::data::Area::new(&area_name, &area_desc, &Vec::new())
}

fn create_quest(client: &reqwest::blocking::Client, area: &crate::data::Area) -> crate::data::Quest {
    let quest_type = crate::data::QuestType::iter()
        .choose(&mut rand::thread_rng())
        .unwrap_or(crate::data::QuestType::Genocide);
    let mut quest = quest_base(client, quest_type, &area.name, &area.description);
    let task = create_task(&quest);
    let giver_prompt = crate::template::quest_giver(&area.name, &area.description, &task);
    let quest_giver = crate::llm::chat(
        client,
        &giver_prompt,
    );
    let quest_desc = crate::llm::chat(
        client,
        &crate::template::quest_description(&area.name, &area.description, &task, &quest_giver),
    );
    quest.giver = quest_giver;
    quest.description = quest_desc;
    quest.area_id = Some(area.id);
    quest
}

fn quest_base(
    client: &reqwest::blocking::Client,
    quest_type: crate::data::QuestType,
    area_name: &str,
    area_desc: &str,
) -> crate::data::Quest {
    match quest_type {
        crate::data::QuestType::Boss => {
            let boss_name = crate::llm::chat(
                client,
                &crate::template::boss_name(area_name, area_desc),
            );
            crate::data::Quest::new_boss("", "", &boss_name)
        },
        crate::data::QuestType::Fetch => {
            let number: u8 = 10; // TODO: rand num
            let item_name = crate::llm::chat(
                client,
                &crate::template::fetch_item(area_name, area_desc, number),
            );
            crate::data::Quest::new_fetch("", "", number, &item_name)
        },
        crate::data::QuestType::Genocide => {
            let number: u8 = 10; // TODO: rand num
            let enemy_name = crate::llm::chat(
                client,
                &crate::template::enemy_name(area_name, area_desc, number),
            );
            crate::data::Quest::new_genocide("", "", number, &enemy_name)
        },
        crate::data::QuestType::Loot => {
            let number: u8 = 10; // TODO: rand num
            let enemy_name = crate::llm::chat(
                client,
                &crate::template::enemy_name(area_name, area_desc, number),
            );
            let item_name = crate::llm::chat(
                client,
                &crate::template::loot_item(area_name, area_desc, number, &enemy_name),
            );
            crate::data::Quest::new_loot("", "", number, &item_name, &enemy_name)
        }
        crate::data::QuestType::Talk => {
            let npc_name = crate::llm::chat(
                client,
                &crate::template::npc_name(area_name, area_desc),
            );
            crate::data::Quest::new_talk("", "", &npc_name)
        }
    }
}

fn create_task(quest: &crate::data::Quest) -> String {
    match quest.the_type {
        crate::data::QuestType::Boss => {
            crate::template::boss_quest(quest.boss.as_ref().unwrap())
        },
        crate::data::QuestType::Fetch => {
            crate::template::fetch_quest(quest.number.unwrap(), quest.item.as_ref().unwrap())
        },
        crate::data::QuestType::Genocide => {
            crate::template::genocide_quest(quest.number.unwrap(), quest.enemy.as_ref().unwrap())
        },
        crate::data::QuestType::Loot => {
            crate::template::loot_quest(quest.number.unwrap(), quest.item.as_ref().unwrap(), quest.enemy.as_ref().unwrap())
        }
        crate::data::QuestType::Talk => {
            crate::template::talk_quest(quest.npc.as_ref().unwrap())
        }
    }
}