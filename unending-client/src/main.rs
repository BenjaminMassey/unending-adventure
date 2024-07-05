mod connection;

fn main() {
    let content = connection::get_random_content();
    println!("{:?}", content.area);
    for quest in content.quests {
        println!("{:?}", quest);
    }
}
