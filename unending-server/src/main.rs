mod data;
mod db;
mod generate;
mod llm;
mod log;
mod server;
mod template;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/get_area_by_uuid", axum::routing::post(server::get_area_by_uuid))
        .route("/get_quest_by_uuid", axum::routing::post(server::get_quest_by_uuid));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}