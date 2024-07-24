mod data;
mod db;
mod generate;
mod llm;
mod log;
mod server;
mod template;

#[cfg(feature = "generate")]
const GENERATE: bool = true;
#[cfg(not(feature = "generate"))]
const GENERATE: bool = false;

#[tokio::main]
async fn main() {
    // Generate content
    if GENERATE {
        std::thread::spawn(|| {
            db::initialize();
            loop {
                let (area, quests) = generate::create_area_with_quests(2);
                db::add_area(&area);
                for quest in &quests {
                    db::add_quest(quest);
                }
            }
        });
    }

    // Host API
    let app = axum::Router::new()
        .route("/get_area_by_uuid", axum::routing::post(server::get_area_by_uuid))
        .route("/get_random_area", axum::routing::get(server::get_random_area))
        .route("/get_quest_by_uuid", axum::routing::post(server::get_quest_by_uuid));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
