use std::time::Duration;

use crate::YodaTaller;

#[derive(serde::Deserialize, Clone)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub swapi: SwapiSettings,
}

#[derive(serde::Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
}

#[derive(serde::Deserialize, Clone)]
pub struct SwapiSettings {
    pub base_url: String,
    pub timeout_milliseconds: u64,
}

impl SwapiSettings {
    pub fn yoda_taller(&self) -> YodaTaller {
        let timeout_duration = Duration::from_millis(self.timeout_milliseconds);
        YodaTaller::new(self.base_url.clone(), timeout_duration)
    }
}
