use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Settings {
    pub port: u16,
    pub database_url: String,
    pub jwt_secret: String,
}

pub fn load() -> Result<Settings, config::ConfigError> {
    dotenvy::dotenv().ok();

    config::Config::builder()
        .add_source(
            config::Environment::default()
        ).build()?
        .try_deserialize::<Settings>()
}