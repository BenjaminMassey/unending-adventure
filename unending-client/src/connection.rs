const SERVER_URL: &str = "http://localhost:3001";

pub struct Content {
    pub area: unending_server::Area,
    pub quests: Vec<unending_server::Quest>,
}

pub fn get_random_content() -> Content {
    let area = get_random_area();
    let quests = get_quests(&area.quest_ids);
    Content{ area, quests }
}

fn get_random_area() -> unending_server::Area {
    let text = reqwest::blocking::get(format!("{SERVER_URL}/get_random_area"))
        .unwrap()
        .text()
        .unwrap();
    let string_area: unending_server::StringArea =
        serde_json::from_str(&text).unwrap();
    string_area.to_area()
}

fn get_quests(quest_ids: &[uuid::Uuid]) -> Vec<unending_server::Quest> {
    let client = reqwest::blocking::Client::new();
    let mut quests = vec![];
    for quest_id in quest_ids {
        let text = client
            .post(format!("{SERVER_URL}/get_quest_by_uuid"))
            .body(format!(r#"{{ "uuid": "{quest_id}" }}"#))
            .header("Content-Type", "application/json")
            .send()
            .unwrap()
            .text()
            .unwrap();
        let string_quest: unending_server::StringQuest =
            serde_json::from_str(&text).unwrap();
        quests.push(string_quest.to_quest());
    }
    quests
}