const SERVER_URL: &str = "http://localhost:3001";

pub fn get_random_area() -> unending_server::Area {
    let text = reqwest::blocking::get(
        format!("{SERVER_URL}/get_random_area")
    ).unwrap().text().unwrap();
    let string_area: unending_server::StringArea =
        serde_json::from_str(&text).unwrap();
    string_area.to_area()
}