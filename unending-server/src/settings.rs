#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub db: DB,
    pub generation: Generation,
    pub api: API,
}

#[derive(serde::Deserialize, Clone)]
pub struct DB {
    pub path: String,
}

#[derive(serde::Deserialize, Clone)]
pub struct Generation {
    pub enabled: bool,
} // TODO: model

#[derive(serde::Deserialize, Clone)]
pub struct API {
    pub address: String,
    pub port: u32,
}


pub fn get_settings() -> Settings {
    toml::from_str(
        &std::fs::read_to_string("settings.toml").expect("Failure to read settings.toml")
    ).expect("Failure to parse settings.toml into Settings")
}