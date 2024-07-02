use std::ops::Deref;

fn fill(path: &str, key_pairs: &[(&str, &str)]) -> String {
    let mut text = std::fs::read_to_string(path).expect("Error reading template file");
    for (key, value) in key_pairs {
        text = text.replace(&format!("[[[{key}]]]"), value);
    }
    text
}

pub fn area_name() -> String {
    fill("./templates/prompts/area_name.txt", &vec![])
}

pub fn area_description(area_name: &str) -> String {
    fill("./templates/prompts/area_desc.txt", &vec![("AREA_NAME", area_name)])
}

pub fn enemy_name(area_name: &str, area_description: &str, enemy_count: u8) -> String {
    let number = enemy_count.to_string();
    fill(
        "./templates/prompts/enemy_name.txt",
        &vec![
            ("AREA_NAME", area_name),
            ("AREA_DESC", area_description),
            ("ENEMY_COUNT", number.deref()),
        ],
    )
}

pub fn boss_name(area_name: &str, area_description: &str) -> String {
    fill(
        "./templates/prompts/boss_name.txt",
        &vec![
            ("AREA_NAME", area_name),
            ("AREA_DESC", area_description),
        ],
    )
}

pub fn fetch_item(area_name: &str, area_description: &str, item_count: u8) -> String {
    let number = item_count.to_string();
    fill(
        "./templates/prompts/fetch_item.txt",
        &vec![
            ("AREA_NAME", area_name),
            ("AREA_DESC", area_description),
            ("ITEM_COUNT", number.deref()),
        ],
    )
}

pub fn loot_item(
    area_name: &str,
    area_description: &str,
    item_count: u8,
    enemy_name: &str,
) -> String {
    let number = item_count.to_string();
    fill(
        "./templates/prompts/loot_item.txt",
        &vec![
            ("AREA_NAME", area_name),
            ("AREA_DESC", area_description),
            ("ITEM_COUNT", number.deref()),
            ("ENEMY_NAME", enemy_name),
        ],
    )
}

pub fn npc_name(area_name: &str, area_description: &str) -> String {
    fill(
        "./templates/prompts/npc_name.txt",
        &vec![
            ("AREA_NAME", area_name),
            ("AREA_DESC", area_description),
        ],
    )
}

pub fn quest_giver(area_name: &str, area_description: &str, quest_task: &str) -> String {
    fill(
        "./templates/prompts/quest_giver.txt",
        &vec![
            ("AREA_NAME", area_name),
            ("AREA_DESC", area_description),
            ("QUEST_TASK", quest_task),
        ],
    )
}

pub fn quest_description(area_name: &str, area_description: &str, quest_task: &str, giver_name: &str) -> String {
    fill(
        "./templates/prompts/quest_desc.txt",
        &vec![
            ("AREA_NAME", area_name),
            ("AREA_DESC", area_description),
            ("QUEST_TASK", quest_task),
            ("GIVER_NAME", giver_name),
        ],
    )
}

pub fn boss_quest(boss_name: &str) -> String {
    fill("./templates/quests/boss.txt", &vec![("BOSS", boss_name)])
}

pub fn fetch_quest(item_count: u8, item_name: &str) -> String {
    let number = item_count.to_string();
    fill(
        "./templates/quests/fetch.txt",
        &vec![
            ("NUMBER", number.deref()),
            ("ITEM", item_name),
        ],
    )
}

pub fn genocide_quest(enemy_count: u8, enemy_name: &str) -> String {
    let number = enemy_count.to_string();
    fill(
        "./templates/quests/genocide.txt",
        &vec![
            ("NUMBER", number.deref()),
            ("ENEMY", enemy_name),
        ],
    )
}

pub fn loot_quest(item_count: u8, item_name: &str, enemy_name: &str) -> String {
    let number = item_count.to_string();
    fill(
        "./templates/quests/loot.txt",
        &vec![
            ("NUMBER", number.deref()),
            ("ITEM", item_name),
            ("ENEMY", enemy_name),
        ],
    )
}

pub fn talk_quest(npc_name: &str) -> String {
    fill("./templates/quests/talk.txt", &vec![("NPC", npc_name)])
}