use std::error::Error;

use chrono::{DateTime, Local};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

use crate::skills::Skill;

#[derive(Debug, Serialize, Deserialize)]
pub struct Workout {
    pub skill: Vec<(Skill, usize)>,
}

/// Saves the workout and time to the database.
pub fn save_workout_to_db(
    connection: &Connection,
    skill_list: Vec<Skill>,
    rep_list: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let current_time = chrono::offset::Local::now();

    let mut workout = Workout { skill: Vec::new() };

    for (skill, reps) in skill_list.iter().zip(rep_list.iter()) {
        let rep_count = reps.parse::<usize>().unwrap_or(0);
        if rep_count != 0 {
            workout.skill.push((skill.to_owned(), rep_count));
        }
    }

    let v = serde_json::to_value(workout)?;

    connection.execute(
        "INSERT INTO workouts VALUES (:timestamp, :workout)",
        (current_time, v),
    )?;

    Ok(())
}

/// Gets the workouts from the database and returns it together with the local time.
pub fn get_workouts_from_db(connection: &Connection) -> Vec<(DateTime<Local>, Workout)> {
    let mut workouts = Vec::new();

    let mut stmt = connection
        .prepare("SELECT * FROM workouts")
        .expect("Something went wrong executing SELECT statement.");

    let workout_iter = stmt
        .query_map([], |row| {
            let v: Workout = serde_json::from_value(row.get_unwrap(1))
                .expect("Error reading workout from database.");
            let time: DateTime<Local> = row.get_unwrap(0);
            Ok((time, v))
        })
        .expect("Reading data failed.");

    for w in workout_iter {
        workouts.push(w.expect("Error reading workout from database."));
    }

    // The newest workouts should come first.
    workouts.reverse();

    workouts
}
