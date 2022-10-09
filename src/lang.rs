#![allow(clippy::use_self)]

use std::{collections::HashMap, error::Error, str::FromStr};

use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::{
    app::RingFitApp,
    settings::Settings,
    skills::{Skill, SkillHashtags},
};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
/// The currently supported languages.
pub enum Languages {
    English,
    German,
}

/// The translation consists of a "Key" and X Values,
/// X = Number of Languages supported.
/// All of which are Strings, of course.
type Translation = (String, String, String);

impl ToString for Languages {
    fn to_string(&self) -> String {
        match self {
            Self::English => "English".into(),
            Self::German => "Deutsch".into(),
        }
    }
}

impl FromStr for Languages {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "English" => Ok(Self::English),
            "Deutsch" | "German" => Ok(Self::German),
            _ => Err(()),
        }
    }
}

/// Switches the display language to the target language.
pub fn switch_language(rfa: &mut RingFitApp, target_language: Languages) {
    rfa.language = target_language;

    let settings = Settings {
        language: target_language,
    };
    let s = serde_json::to_string_pretty(&settings).expect("Could not serialize json to string");

    std::fs::write("./settings/settings.json", s).expect("Could not write to settings.json");

    let (skill_hashmap, hashtag_hashmap, menu_hashmap) =
        get_language_hashmaps(&rfa.db_connection, target_language);

    rfa.skill_names = skill_hashmap;
    rfa.hashtag_names = hashtag_hashmap;
    rfa.menu_names = menu_hashmap;
}

#[must_use]
/// Gets all of the translation hashmaps of a specified language.
pub fn get_language_hashmaps(
    connection: &Connection,
    target_language: Languages,
) -> (
    HashMap<Skill, String>,
    HashMap<SkillHashtags, String>,
    HashMap<String, String>,
) {
    let all_skills = Skill::get_all_skills(connection);
    let all_hashtags = SkillHashtags::get_all_hashtags();
    let all_menus = get_all_translations().expect("Could not read translations from database.");

    let mut skill_hashmap = HashMap::new();
    for skill in all_skills {
        let name = skill.get_translated_name(connection, &target_language);
        skill_hashmap.insert(skill, name);
    }

    let mut hashtag_hashmap = HashMap::new();
    for hashtag in all_hashtags {
        let name = hashtag.get_translated_name(connection, &target_language);
        hashtag_hashmap.insert(hashtag, name);
    }

    let mut menu_hashmap = HashMap::new();
    for item in all_menus {
        match target_language {
            Languages::English => menu_hashmap.insert(item.0, item.1),
            Languages::German => menu_hashmap.insert(item.0, item.2),
        };
    }

    (skill_hashmap, hashtag_hashmap, menu_hashmap)
}

/// Gets every translation in the translations.json file
/// and converts it into a Vector of Translations, aka (String, String, String).
pub fn get_all_translations() -> Result<Vec<Translation>, Box<dyn Error>> {
    let file_content = include_str!("../assets/translations.json");

    let v: Value = serde_json::from_str(file_content)?;

    let mut translations = Vec::new();

    for (key, value) in v.as_object().unwrap_or(&Map::new()) {
        translations.push((
            key.clone(),
            // .to_string() would leave the "" unchanged,
            // .as_str() removes them but we need to unwrap and convert after.
            value.as_array().unwrap_or(&Vec::new())[0]
                .as_str()
                .unwrap_or("")
                .into(),
            value.as_array().unwrap_or(&Vec::new())[1]
                .as_str()
                .unwrap_or("")
                .into(),
        ));
    }

    Ok(translations)
}

/// Gets a translated string directly from the database given the target language and the key value.
pub fn get_string(
    connection: &Connection,
    language: &Languages,
    key: String,
) -> Result<String, Box<dyn Error>> {
    let index = match language {
        Languages::English => "en",
        Languages::German => "de",
    };

    let mut stmt = connection.prepare(&format!(
        "SELECT {} FROM translations WHERE key = :key",
        index
    ))?;

    let translation: String = stmt.query_row([key], |r| r.get(0))?;

    Ok(translation)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_lang_string_conv() {
        let german_string = "Deutsch";
        let english_string = "English";
        let invalid_string = "Something else";

        assert_eq!(german_string, &Languages::German.to_string());
        assert_eq!(english_string, &Languages::English.to_string());

        assert_eq!(
            Languages::from_str(german_string).unwrap(),
            Languages::German
        );
        assert_eq!(
            Languages::from_str(english_string).unwrap(),
            Languages::English
        );
        assert_eq!(Languages::from_str(invalid_string), Err(()));
    }
}
