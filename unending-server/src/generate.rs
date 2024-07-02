use crate::data;
use crate::llm;
use crate::template;

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
    // TODO: generate quest type
    let enemy_name = llm::gpt4all_chat(
        &template::enemy_name(area_name, area_desc),
        10,
    ).unwrap(); // TODO: only if genocide
    let task = format!("kill 10 {enemy_name}s"); // TODO: real
    let quest_giver = llm::gpt4all_chat(
        &template::quest_giver(area_name, area_desc, &task),
        10,
    ).unwrap();
    let quest_desc = llm::gpt4all_chat(
        &template::quest_description(area_name, area_desc, &task, &quest_giver),
        300,
    ).unwrap();

    data::Quest {
        the_type: data::QuestType::Genocide, // TODO: variety
        giver: quest_giver,
        description: quest_desc,
        enemy_name: Some(enemy_name),
    }
}