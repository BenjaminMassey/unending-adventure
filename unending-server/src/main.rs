mod data;
mod generate;
mod llm;
mod log;
mod template;

fn main() {
    let area = generate::create_area(2);
    println!(
        "Area Name: {}\n\nArea Description: {}",
        area.name, area.description,
    );
    for quest in area.quests {
        println!("\n{:?}", quest);
    }
}
