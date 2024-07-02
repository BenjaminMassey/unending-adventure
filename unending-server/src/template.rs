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

pub fn enemy_name(area_name: &str, area_description: &str) -> String {
    fill(
        "./templates/prompts/enemy_name.txt",
        &vec![("AREA_NAME", area_name), ("AREA_DESC", area_description)],
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