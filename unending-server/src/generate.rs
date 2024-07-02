use crate::data;
use crate::llm;
use crate::template;

use rand::seq::IteratorRandom;
use strum::IntoEnumIterator;

pub fn create_area(quest_count: usize) -> data::Area {
    let area_name = llm::gpt4all_chat(
        &template::area_name(),
        20,
    ).unwrap();
    let area_desc = llm::gpt4all_chat(
        &template::area_description(&area_name),
        200,
    ).unwrap();
    let mut quests: Vec<data::Quest> = vec![];
    for _ in 0..quest_count {
        quests.push(create_quest(&area_name, &area_desc));
    }
    data::Area{
        name: area_name,
        description: area_desc,
        quests,
    }
}

fn create_quest(area_name: &str, area_desc: &str) -> data::Quest {
    let quest_type = data::QuestType::iter()
        .choose(&mut rand::thread_rng())
        .unwrap();
    let mut quest = quest_base(quest_type, area_name, area_desc);
    let task = create_task(&quest);
    let quest_giver = llm::gpt4all_chat(
        &template::quest_giver(area_name, area_desc, &task),
        10,
    ).unwrap();
    let quest_desc = llm::gpt4all_chat(
        &template::quest_description(area_name, area_desc, &task, &quest_giver),
        300,
    ).unwrap();
    quest.giver = quest_giver;
    quest.description = quest_desc;

    quest
}

fn quest_base(quest_type: data::QuestType, area_name: &str, area_desc: &str) -> data::Quest {
    match quest_type {
        data::QuestType::Boss => {
            let boss_name = llm::gpt4all_chat(
                &template::boss_name(area_name, area_desc),
                10,
            ).unwrap();
            data::Quest::new_boss("", "", &boss_name)
        },
        data::QuestType::Fetch => {
            let number: u8 = 10; // TODO: rand num
            let item_name = llm::gpt4all_chat(
                &template::fetch_item(area_name, area_desc, number),
                10,
            ).unwrap();
            data::Quest::new_fetch("", "", number, &item_name)
        },
        data::QuestType::Genocide => {
            let number: u8 = 10; // TODO: rand num
            let enemy_name = llm::gpt4all_chat(
                &template::enemy_name(area_name, area_desc, number),
                10,
            ).unwrap();
            data::Quest::new_genocide("", "", number, &enemy_name)
        },
        data::QuestType::Loot => {
            let number: u8 = 10; // TODO: rand num
            let enemy_name = llm::gpt4all_chat(
                &template::enemy_name(area_name, area_desc, number),
                10,
            ).unwrap();
            let item_name = llm::gpt4all_chat(
                &template::loot_item(area_name, area_desc, number, &enemy_name),
                10,
            ).unwrap();
            data::Quest::new_loot("", "", number, &item_name, &enemy_name)
        }
        data::QuestType::Talk => {
            let npc_name = llm::gpt4all_chat(
                &template::npc_name(area_name, area_desc),
                10,
            ).unwrap();
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