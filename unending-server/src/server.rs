use crate::db;

use std::str::FromStr;

#[derive(serde::Deserialize)]
pub struct UuidRequest {
    uuid: String,
}

#[derive(serde::Serialize)]
pub struct Area {
    name: String,
    description: String,
} // TODO: can maybe use data::Area if reworked?

pub async fn get_area_by_uuid(
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
pub struct Quest {
    the_type: String,
    giver: String,
    description: String,
    number: String,
    enemy: String,
    item: String,
    boss: String,
    npc: String,
} // TODO: can maybe use data::Quest if reworked?

pub async fn get_quest_by_uuid(
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