use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::lang::Languages;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub language: Languages,
}

/// Loads the settings from the settings.json file into a the Settings struct.
pub fn load_settings() -> Result<Settings, Box<dyn Error>> {
    let file_content = std::fs::read_to_string("./settings/settings.json")?;

    let settings: Settings = serde_json::from_str(&file_content)?;

    Ok(settings)
}
