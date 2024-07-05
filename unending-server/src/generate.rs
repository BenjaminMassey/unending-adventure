use crate::data;
use crate::llm;
use crate::template;

use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;

pub fn create_area_with_quests(
    quest_count: usize
) -> (data::Area, Vec<data::Quest>) {
    let mut area = create_area();
    let mut quests: Vec<data::Quest> = vec![];
    for _ in 0..quest_count {
        quests.push(create_quest(&area));
    }
    let quest_ids: Vec<uuid::Uuid> = quests
        .iter()
        .map(|q| q.id)
        .collect();
    area.quest_ids = quest_ids;
    (area, quests)
}

fn create_area() -> data::Area {
    let area_name = llm::gpt4all_chat_force(
        &template::area_name(),
        20,
    );
    let area_desc = llm::gpt4all_chat_force(
        &template::area_description(&area_name),
        200,
    );
    data::Area::new(&area_name, &area_desc, &Vec::new())
}

fn create_quest(area: &data::Area) -> data::Quest {
    let quest_type = data::QuestType::iter()
        .choose(&mut rand::thread_rng())
        .unwrap_or(data::QuestType::Genocide);
    let mut quest = quest_base(quest_type, &area.name, &area.description);
    let task = create_task(&quest);
    let giver_prompt = template::quest_giver(&area.name, &area.description, &task);
    let quest_giver = llm::gpt4all_chat_force(
        &giver_prompt,
        10,
    );
    let quest_desc = llm::gpt4all_chat_force(
        &template::quest_description(&area.name, &area.description, &task, &quest_giver),
        300,
    );
    quest.giver = quest_giver;
    quest.description = quest_desc;
    quest.area_id = Some(area.id);
    quest
}

fn quest_base(
    quest_type: data::QuestType,
    area_name: &str,
    area_desc: &str,
) -> data::Quest {
    match quest_type {
        data::QuestType::Boss => {
            let boss_name = llm::gpt4all_chat_force(
                &template::boss_name(area_name, area_desc),
                10,
            );
            data::Quest::new_boss("", "", &boss_name)
        },
        data::QuestType::Fetch => {
            let number: u8 = 10; // TODO: rand num
            let item_name = llm::gpt4all_chat_force(
                &template::fetch_item(area_name, area_desc, number),
                10,
            );
            data::Quest::new_fetch("", "", number, &item_name)
        },
        data::QuestType::Genocide => {
            let number: u8 = 10; // TODO: rand num
            let enemy_name = llm::gpt4all_chat_force(
                &template::enemy_name(area_name, area_desc, number),
                10,
            );
            data::Quest::new_genocide("", "", number, &enemy_name)
        },
        data::QuestType::Loot => {
            let number: u8 = 10; // TODO: rand num
            let enemy_name = llm::gpt4all_chat_force(
                &template::enemy_name(area_name, area_desc, number),
                10,
            );
            let item_name = llm::gpt4all_chat_force(
                &template::loot_item(area_name, area_desc, number, &enemy_name),
                10,
            );
            data::Quest::new_loot("", "", number, &item_name, &enemy_name)
        }
        data::QuestType::Talk => {
            let npc_name = llm::gpt4all_chat_force(
                &template::npc_name(area_name, area_desc),
                10,
            );
            data::Quest::new_talk("", "", &npc_name)
        }
    }
}

fn create_task(quest: &data::Quest) -> String {
    match quest.the_type {
        data::QuestType::Boss => {
            template::boss_quest(quest.boss.as_ref().unwrap())
        },
        data::QuestType::Fetch => {
            template::fetch_quest(quest.number.unwrap(), quest.item.as_ref().unwrap())
        },
        data::QuestType::Genocide => {
            template::genocide_quest(quest.number.unwrap(), quest.enemy.as_ref().unwrap())
        },
        data::QuestType::Loot => {
            template::loot_quest(quest.number.unwrap(), quest.item.as_ref().unwrap(), quest.enemy.as_ref().unwrap())
        }
        data::QuestType::Talk => {
            template::talk_quest(quest.npc.as_ref().unwrap())
        }
    }
}