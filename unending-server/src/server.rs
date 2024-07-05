use crate::data;
use crate::db;

use std::str::FromStr;

// TODO: more calls and better general usability

pub async fn get_area_by_uuid(
    axum::Json(payload): axum::Json<data::UuidRequest>,
) -> impl axum::response::IntoResponse {
    let area = db::get_area_by_uuid(
        uuid::Uuid::from_str(&payload.uuid).unwrap()
    );
    let string_area = data::StringArea::from_area(&area);
    (axum::http::StatusCode::OK, axum::Json(string_area))
}

pub async fn get_random_area() -> impl axum::response::IntoResponse {
    let area = db::get_random_area();
    let string_area = data::StringArea::from_area(&area);
    (axum::http::StatusCode::OK, axum::Json(string_area))
}

pub async fn get_quest_by_uuid(
    axum::Json(payload): axum::Json<data::UuidRequest>,
) -> impl axum::response::IntoResponse {
    let quest = db::get_quest_by_uuid(
        uuid::Uuid::from_str(&payload.uuid).unwrap()
    );
    let string_quest = data::StringQuest::from_quest(&quest);
    (axum::http::StatusCode::OK, axum::Json(string_quest))
}