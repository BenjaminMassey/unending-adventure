mod data;
mod db;
mod generate;
mod llm;
mod log;
mod template;

use std::str::FromStr;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/get_area_by_uuid", axum::routing::post(get_area_by_uuid))
        .route("/get_quest_by_uuid", axum::routing::post(get_quest_by_uuid));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



#[derive(serde::Deserialize)]
struct UuidRequest {
    uuid: String,
}

#[derive(serde::Serialize)]
struct Area {
    name: String,
    description: String,
} // TODO: can maybe use data::Area if reworked?

async fn get_area_by_uuid(
    axum::Json(payload): axum::Json<UuidRequest>,
) -> impl axum::response::IntoResponse {
    let data_area = db::get_area(
        uuid::Uuid::from_str(&payload.uuid).unwrap()
    );
    let area = Area {
        name: data_area.name,
        description: data_area.description,
    };
    (axum::http::StatusCode::CREATED, axum::Json(area))
}

#[derive(serde::Serialize)]
struct Quest {
    the_type: String,
    giver: String,
    description: String,
    number: String,
    enemy: String,
    item: String,
    boss: String,
    npc: String,
} // TODO: can maybe use data::Quest if reworked?

async fn get_quest_by_uuid(
    axum::Json(payload): axum::Json<UuidRequest>,
) -> impl axum::response::IntoResponse {
    let data_quest = db::get_quest(
        uuid::Uuid::from_str(&payload.uuid).unwrap()
    );
    let quest = Quest { // TODO: better
        the_type: format!("{:?}", data_quest.the_type),
        giver: data_quest.giver,
        description: data_quest.description,
        number: {if let Some(number) = data_quest.number { number.to_string() } else { String::new() }},
        enemy: {if let Some(enemy) = &data_quest.enemy { enemy.to_owned() } else { String::new() }},
        item: {if let Some(item) = &data_quest.item { item.to_owned() } else { String::new() }},
        boss: {if let Some(boss) = &data_quest.boss { boss.to_owned() } else { String::new() }},
        npc: {if let Some(npc) = &data_quest.npc { npc.to_owned() } else { String::new() }},
    };
    (axum::http::StatusCode::CREATED, axum::Json(quest))
}