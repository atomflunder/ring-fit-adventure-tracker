use std::collections::HashMap;

use egui::Context;
use egui_extras::RetainedImage;
use rusqlite::Connection;

use crate::lang::{get_language_hashmaps, Languages};
use crate::menu::{display_menu, Menu};
use crate::settings::load_settings;
use crate::skills::{Skill, SkillHashtags};

pub struct RingFitApp {
    pub skills: Vec<Skill>,
    pub input_reps: Vec<String>,
    pub menu: Option<Menu>,
    pub language: Languages,
    // We load some images on startup.
    pub images: Vec<RetainedImage>,
    // These are set so that we dont have to read them from the database every time.
    // Can just set them every time we switch languages and then look them up in memory.
    pub hashtag_names: HashMap<SkillHashtags, String>,
    pub skill_names: HashMap<Skill, String>,
    pub menu_names: HashMap<String, String>,
    // Same here, we dont want to reconnect every time.
    pub db_connection: Connection,
}

impl Default for RingFitApp {
    fn default() -> Self {
        let settings = load_settings().expect("Could not read settings.json file.");

        let connection =
            Connection::open("./db/database.db").expect("Could not open connection to database.");

        // Getting every skill available.
        let all_skills = Skill::get_all_skills(&connection);

        // Getting the translations to save in the hashmaps.
        let (skill_hashmap, hashtag_hashmap, menu_hashmap) =
            get_language_hashmaps(&connection, settings.language);

        // Loading some icons to display them later on.
        let image_bytes = vec![
            RetainedImage::from_image_bytes(
                "icon_arms.png",
                include_bytes!("../assets/icon_arms.png"),
            )
            .expect("Could not read icon_arms.png"),
            RetainedImage::from_image_bytes(
                "icon_abs.png",
                include_bytes!("../assets/icon_abs.png"),
            )
            .expect("Could not read icon_abs.png"),
            RetainedImage::from_image_bytes(
                "icon_legs.png",
                include_bytes!("../assets/icon_legs.png"),
            )
            .expect("Could not read icon_legs.png"),
            RetainedImage::from_image_bytes(
                "icon_yoga.png",
                include_bytes!("../assets/icon_yoga.png"),
            )
            .expect("Could not read icon_yoga.png"),
        ];

        Self {
            input_reps: vec!["".into(); all_skills.len()],
            skills: all_skills,
            menu_names: menu_hashmap,
            hashtag_names: hashtag_hashmap,
            skill_names: skill_hashmap,
            images: image_bytes,
            menu: None,
            language: settings.language,
            db_connection: connection,
        }
    }
}

impl eframe::App for RingFitApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        display_menu(self, ctx);
    }
}
