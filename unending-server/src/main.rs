mod data;
mod db;
mod generate;
mod llm;
mod log;
mod template;

fn main() {
    // Generate data
    let area = generate::create_area(2);
    
    // Store data
    db::initialize();
    let area_id = area.id;
    db::add_area(&area);
    let mut quest_ids = vec![];
    for quest in &area.quests {
        quest_ids.push(quest.id);
        db::add_quest(quest);
    }

    // Read data
    println!("{:?}", db::get_area(area_id));
    for quest_id in quest_ids {
        println!("\n{:?}", db::get_quest(quest_id));
    }
}
