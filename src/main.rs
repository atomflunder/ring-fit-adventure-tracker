use std::{
    error::Error,
    fs::{create_dir_all, write, File},
};

use eframe::IconData;
use egui::Vec2;
use rfa_tracker::{app::RingFitApp, db::setup_db, lang::Languages, settings::Settings};

/// Sets up the required files and folders for first time usage.
fn first_time_setup() -> Result<(), Box<dyn Error>> {
    // Trying to see if the file exists and if not, create it and run first time setup.
    match File::open("./settings/settings.json") {
        Ok(_) => (),
        Err(_) => {
            create_dir_all("./settings/")?;
            File::create("./settings/settings.json")?;
            let settings = Settings {
                language: Languages::English,
            };
            let s = serde_json::to_string_pretty(&settings)?;

            write("./settings/settings.json", s)?;
        }
    };

    match File::open("./db/database.db") {
        Ok(_) => (),
        Err(_) => {
            create_dir_all("./db/")?;
            File::create("./db/database.db")?;
            setup_db()?;
        }
    };

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    first_time_setup()?;

    let image_bytes = include_bytes!("../assets/icon_arms.png");
    let image_data = image::load_from_memory(image_bytes)?.to_rgba8();
    let (image_height, image_width) = image_data.dimensions();

    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2::new(750., 1000.)),
        icon_data: Some(IconData {
            rgba: image_data.into_raw(),
            width: image_height,
            height: image_width,
        }),

        ..Default::default()
    };

    eframe::run_native(
        "Ring Fit Adventure Tracker",
        options,
        Box::new(|_cc| Box::new(RingFitApp::default())),
    );

    Ok(())
}
