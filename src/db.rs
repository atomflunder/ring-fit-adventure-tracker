use std::error::Error;

use rusqlite::Connection;

use crate::{lang::get_all_translations, skills::all_skills_default};

/// Sets up the database for first time usage.
/// Not really needed after starting the program for the first time.
pub fn setup_db() -> Result<(), Box<dyn Error>> {
    let connection = Connection::open("./db/database.db")?;

    // First we create the translations table.
    connection.execute(
        "CREATE TABLE IF NOT EXISTS translations (key TEXT UNIQUE, en TEXT, de TEXT)",
        (),
    )?;

    // And then populate it with the contents of translations.json.
    for translation in get_all_translations()? {
        connection.execute(
            "INSERT OR IGNORE INTO translations VALUES (:key, :en, :de)",
            (translation.0, translation.1, translation.2),
        )?;
    }

    connection.execute(
        "CREATE TABLE IF NOT EXISTS workouts
            (timestamp DATE, workout BLOB)",
        (),
    )?;

    connection.execute(
        "
            CREATE TABLE IF NOT EXISTS skills 
            (name TEXT UNIQUE, type TEXT, hits TEXT, damage TEXT, unlock TEXT, hashtag TEXT, recharge TEXT, goal_reps INTEGER, completed_reps INTEGER)
    ",
        (),
    )?;

    for skill in all_skills_default() {
        connection.execute(
            "INSERT OR IGNORE INTO skills VALUES (:name, :type, :hits, :damage, :unlock, :hashtag, :recharge, :goal_reps, :completed_reps)",
            (
                skill.name,
                skill.skill_type.to_string(),
                skill.hits.to_string(),
                // We have to convert the array of usizes into a String, separated by commas.
                skill.damage.iter().map(|i| i.to_string() + ",").collect::<String>(),
                skill.unlocks.iter().map(|i| i.to_string() + ",").collect::<String>(),
                skill.hashtags.iter().map(|i| i.to_string() + ",").collect::<String>(),
                skill.recharge_time.iter().map(|i| i.to_string() + ",").collect::<String>(),
                skill.goal_reps,
                skill.completed_reps,
            ),
        )?;
    }

    Ok(())
}
