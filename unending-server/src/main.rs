mod data;
mod db;
mod generate;
mod llm;
mod log;
mod server;
mod settings;
mod template;

#[tokio::main]
async fn main() {
    // Gather settings
    let settings = settings::get_settings();

    // Generate content
    if settings.generation.enabled {
        let gen_settings = settings.clone();
        std::thread::spawn(move || {
            let client = reqwest::blocking::Client::new();
            db::initialize(&gen_settings);
            loop {
                let (area, quests) = generate::create_area_with_quests(&client, 2);
                db::add_area(&gen_settings, &area);
                for quest in &quests {
                    db::add_quest(&gen_settings, quest);
                }
            }
        });
    }

    // Host API
    let state = std::sync::Arc::new(
        server::AxumState::new(&settings)
    );
    let app = axum::Router::new()
        .route("/get_area_by_uuid", axum::routing::post(server::get_area_by_uuid))
        .route("/get_random_area", axum::routing::get(server::get_random_area))
        .route("/get_quest_by_uuid", axum::routing::post(server::get_quest_by_uuid))
        .with_state(state);
    let url = format!("{}:{}", &settings.api.address, settings.api.port);
    let listener = tokio::net::TcpListener::bind(url).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}