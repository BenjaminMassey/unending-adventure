mod data;
mod db;
mod generate;
mod llm;
mod log;
mod server;
mod template;

const GENERATE: bool = false;

#[tokio::main]
async fn main() {
    // Generate content
    if GENERATE {
        std::thread::spawn(|| {
            db::initialize();
            loop {
                let area = generate::create_area(1);
                db::add_area(&area);
                for quest in &area.quests {
                    db::add_quest(quest);
                }
            }
        });
    }

    // Host API
    let app = axum::Router::new()
        .route("/get_area_by_uuid", axum::routing::post(server::get_area_by_uuid))
        .route("/get_quest_by_uuid", axum::routing::post(server::get_quest_by_uuid));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}