
use std::str::FromStr;

pub struct AxumState {
    pub settings: crate::settings::Settings,
}
impl AxumState {
    pub fn new(settings: &crate::settings::Settings) -> Self {
        Self {
            settings: settings.clone()
        }
    }
}

pub async fn get_area_by_uuid(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AxumState>>,
    axum::Json(payload): axum::Json<crate::data::UuidRequest>,
) -> impl axum::response::IntoResponse {
    let area = crate::db::get_area_by_uuid(
        &state.settings,
        uuid::Uuid::from_str(&payload.uuid).unwrap()
    );
    let string_area = crate::data::StringArea::from_area(&area);
    (axum::http::StatusCode::OK, axum::Json(string_area))
}

pub async fn get_random_area(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AxumState>>,
) -> impl axum::response::IntoResponse {
    let area = crate::db::get_random_area(&state.settings);
    let string_area = crate::data::StringArea::from_area(&area);
    (axum::http::StatusCode::OK, axum::Json(string_area))
}

pub async fn get_quest_by_uuid(
    axum::extract::State(state): axum::extract::State<std::sync::Arc<AxumState>>,
    axum::Json(payload): axum::Json<crate::data::UuidRequest>,
) -> impl axum::response::IntoResponse {
    let quest = crate::db::get_quest_by_uuid(
        &state.settings,
        uuid::Uuid::from_str(&payload.uuid).unwrap()
    );
    let string_quest = crate::data::StringQuest::from_quest(&quest);
    (axum::http::StatusCode::OK, axum::Json(string_quest))
}