use std::{error::Error, hash::Hash, str::FromStr};

use rusqlite::{
    types::{FromSql, FromSqlError, ValueRef},
    Connection,
};
use serde::{Deserialize, Serialize};

use crate::lang::{get_string, Languages};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub skill_type: SkillTypes,
    pub hits: SkillHits,
    pub damage: [usize; 4],
    pub unlocks: [usize; 4],
    pub hashtags: [SkillHashtags; 3],
    pub recharge_time: [usize; 4],
    pub goal_reps: usize,
    pub completed_reps: usize,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
/// All of the 4 different skill types in game.
pub enum SkillTypes {
    Arms,
    Core,
    Legs,
    Yoga,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
/// The different effects skills can have in game, they can hit X enemies or heal the player.
pub enum SkillHits {
    One,
    Three,
    Five,
    Heal,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Serialize, Deserialize)]
/// All hashtags found in game, these describe what muscle groups get worked when doing an excercise.
/// A skill can have up to three hashtags and always has at least one.
pub enum SkillHashtags {
    Empty,
    Chest,
    UpperArms,
    Shoulders,
    Trapezius,
    Core,
    Posture,
    Legs,
    Glutes,
    LowerBody,
    Abs,
    Waist,
    Stamina,
    Back,
    Flexibility,
    Aerobic,
}

impl std::fmt::Display for SkillTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for SkillHits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::fmt::Display for SkillHashtags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Don't ask me why some hashtags in game have spaces in them.
            Self::UpperArms => write!(f, "#Upper Arms"),
            Self::LowerBody => write!(f, "#Lower Body"),
            _ => write!(f, "#{:?}", self),
        }
    }
}

// We just compare skill names for equality instead of the whole skill,
// because that *could* change while running the program.
// For example while updating skill reps.
impl PartialEq for Skill {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Skill {
    fn assert_receiver_is_total_eq(&self) {}
}

// Need to impl Hash manually as we can't derive it when impl Eq manually.
impl Hash for Skill {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl SkillHashtags {
    #[must_use]
    /// Gets the display name for the hashtag, translated.
    pub fn get_translated_name(&self, connection: &Connection, language: &Languages) -> String {
        let key = match self {
            Self::Empty => "hashtag_empty",
            Self::Chest => "hashtag_chest",
            Self::UpperArms => "hashtag_upper_arms",
            Self::Shoulders => "hashtag_shoulders",
            Self::Trapezius => "hashtag_trapezius",
            Self::Core => "hashtag_core",
            Self::Posture => "hashtag_posture",
            Self::Legs => "hashtag_legs",
            Self::Glutes => "hashtag_glutes",
            Self::LowerBody => "hashtag_lower_body",
            Self::Abs => "hashtag_abs",
            Self::Waist => "hashtag_waist",
            Self::Stamina => "hashtag_stamina",
            Self::Back => "hashtag_back",
            Self::Flexibility => "hashtag_flexibility",
            Self::Aerobic => "hashtag_aerobic",
        };

        get_string(connection, language, key.into()).unwrap_or_else(|_| "Invalid".into())
    }

    #[must_use]
    /// Gets all the hashtags in a Vec.
    pub fn get_all_hashtags() -> Vec<Self> {
        vec![
            Self::Empty,
            Self::Chest,
            Self::UpperArms,
            Self::Shoulders,
            Self::Trapezius,
            Self::Core,
            Self::Posture,
            Self::Legs,
            Self::Glutes,
            Self::LowerBody,
            Self::Abs,
            Self::Waist,
            Self::Stamina,
            Self::Back,
            Self::Flexibility,
            Self::Aerobic,
        ]
    }
}

impl FromSql for SkillTypes {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            ValueRef::Text(t) => match std::str::from_utf8(t).unwrap_or("") {
                "Arms" => Ok(Self::Arms),
                "Core" => Ok(Self::Core),
                "Legs" => Ok(Self::Legs),
                _ => Ok(Self::Yoga),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

/// Some type conversions needed.
impl FromSql for SkillHits {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            ValueRef::Text(t) => match std::str::from_utf8(t).unwrap_or("") {
                "One" => Ok(Self::One),
                "Three" => Ok(Self::Three),
                "Five" => Ok(Self::Five),
                _ => Ok(Self::Heal),
            },
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl FromStr for SkillTypes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Arms" => Ok(Self::Arms),
            "Core" => Ok(Self::Core),
            "Legs" => Ok(Self::Legs),
            "Yoga" => Ok(Self::Yoga),
            _ => Err(()),
        }
    }
}

impl FromStr for SkillHashtags {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | "#Empty" => Ok(Self::Empty),
            "#Chest" => Ok(Self::Chest),
            "#Upper Arms" => Ok(Self::UpperArms),
            "#Shoulders" => Ok(Self::Shoulders),
            "#Trapezius" => Ok(Self::Trapezius),
            "#Core" => Ok(Self::Core),
            "#Posture" => Ok(Self::Posture),
            "#Legs" => Ok(Self::Legs),
            "#Glutes" => Ok(Self::Glutes),
            "#Abs" => Ok(Self::Abs),
            "#Stamina" => Ok(Self::Stamina),
            "#Back" => Ok(Self::Back),
            "#Flexibility" => Ok(Self::Flexibility),
            "#Aerobic" => Ok(Self::Aerobic),
            "#Waist" => Ok(Self::Waist),
            "#Lower Body" => Ok(Self::LowerBody),
            _ => Err(()),
        }
    }
}

impl FromStr for SkillHits {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "One" => Ok(Self::One),
            "Three" => Ok(Self::Three),
            "Five" => Ok(Self::Five),
            "Heal" => Ok(Self::Heal),
            _ => Err(()),
        }
    }
}

impl Skill {
    #[must_use]
    /// Gets the reps needed until you reach your goal, or 0 if it is already reached.
    pub fn get_reps_until_goal(&self) -> usize {
        (self.goal_reps as isize - self.completed_reps as isize).max(0) as usize
    }

    #[must_use]
    /// This function gets the percentage of completion and stops at 100%.
    pub fn get_rep_percent(&self) -> f64 {
        (self.completed_reps as f64 / self.goal_reps as f64).min(1.0) * 100.0
    }

    #[must_use]
    /// This function is the same as `get_rep_percent` but does not stop at 100%.
    pub fn get_rep_percent_uncapped(&self) -> f64 {
        (self.completed_reps as f64 / self.goal_reps as f64) * 100.0
    }

    #[must_use]
    /// Gets the translated name of a skill.
    pub fn get_translated_name(&self, connection: &Connection, language: &Languages) -> String {
        let mut key = "skill_".to_string();
        key.push_str(
            self.name
                .to_ascii_lowercase()
                .replace(' ', "_")
                .replace('&', "and")
                .as_str(),
        );

        get_string(connection, language, key).unwrap_or_else(|_| "Invalid".into())
    }

    #[must_use]
    /// Gets all skills in a Vec.
    pub fn get_all_skills(connection: &Connection) -> Vec<Self> {
        let mut skills = Vec::new();

        let mut stmt = connection
            .prepare("SELECT * FROM skills")
            .expect("Something went wrong executing SELECT statement.");

        let skill_iter = stmt
            .query_map([], |row| {
                Ok(Self {
                    name: row.get_unwrap(0),
                    skill_type: row.get_unwrap(1),
                    hits: row.get_unwrap(2),
                    // We have to convert the String into an array of usizes again.
                    damage: {
                        let result_str: String = row.get_unwrap(3);
                        // We will always have 4 values separated by commas in the string.
                        // Except if the user screws something up manually in the database or something.
                        // So first we initialise an empty result array and then fill it manually.
                        let mut result_vec = [0, 0, 0, 0];
                        // We filter empty results, the damage numbers will never be empty but the hashtags below might be.
                        for (i, j) in result_str.split(',').filter(|s| !s.is_empty()).enumerate() {
                            let num = j.parse::<usize>().unwrap_or(0);
                            result_vec[i] = num;
                        }
                        result_vec
                    },
                    unlocks: {
                        let result_str: String = row.get_unwrap(4);
                        let mut result_vec = [0, 0, 0, 0];
                        for (i, j) in result_str.split(',').filter(|s| !s.is_empty()).enumerate() {
                            let num = j.parse::<usize>().unwrap_or(0);
                            result_vec[i] = num;
                        }
                        result_vec
                    },
                    hashtags: {
                        let result_str: String = row.get_unwrap(5);
                        let mut result_vec: [SkillHashtags; 3] = [
                            SkillHashtags::Empty,
                            SkillHashtags::Empty,
                            SkillHashtags::Empty,
                        ];
                        for (i, j) in result_str.split(',').filter(|s| !s.is_empty()).enumerate() {
                            result_vec[i] =
                                SkillHashtags::from_str(j).unwrap_or(SkillHashtags::Empty);
                        }
                        result_vec
                    },
                    recharge_time: {
                        let result_str: String = row.get_unwrap(6);
                        let mut result_vec = [0, 0, 0, 0];
                        for (i, j) in result_str.split(',').filter(|s| !s.is_empty()).enumerate() {
                            let num = j.parse::<usize>().unwrap_or(0);
                            result_vec[i] = num;
                        }
                        result_vec
                    },
                    goal_reps: row.get_unwrap(7),
                    completed_reps: row.get_unwrap(8),
                })
            })
            .expect("Reading data failed.");

        for skill in skill_iter {
            skills.push(skill.expect("Could not read skill data from database."));
        }

        skills
    }

    /// Increases the reps for a skill by X.
    pub fn update_reps(
        &self,
        connection: &Connection,
        reps_today: usize,
    ) -> Result<(), Box<dyn Error>> {
        connection.execute(
            "UPDATE skills SET completed_reps = completed_reps + :reps_today WHERE name = :name",
            (reps_today, self.name.clone()),
        )?;

        Ok(())
    }

    /// Sets the reps for a skill to X.
    pub fn set_reps(
        &self,
        connection: &Connection,
        total_reps: usize,
    ) -> Result<(), Box<dyn Error>> {
        connection.execute(
            "UPDATE skills SET completed_reps = :total_reps WHERE name = :name",
            (total_reps, self.name.clone()),
        )?;

        Ok(())
    }
}

#[must_use]
#[allow(clippy::too_many_lines)]
/// Gets all skills in default form, used when setting up the database for the first time.
/// This would probably be make more sense in a json file.
pub fn all_skills_default() -> Vec<Skill> {
    [
        Skill {
            name: "Front Press".into(),
            skill_type: SkillTypes::Arms,
            hits: SkillHits::Three,
            damage: [25, 320, 390, 745],
            unlocks: [5, 144, 148, 286],
            hashtags: [
                SkillHashtags::Chest,
                SkillHashtags::Empty,
                SkillHashtags::Empty,
            ],
            recharge_time: [2, 3, 4, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Overhead Press".into(),
            skill_type: SkillTypes::Arms,
            hits: SkillHits::One,
            damage: [30, 350, 655, 1000],
            unlocks: [1, 104, 201, 286],
            hashtags: [
                SkillHashtags::UpperArms,
                SkillHashtags::Chest,
                SkillHashtags::Shoulders,
            ],
            recharge_time: [1, 2, 3, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Back Press".into(),
            skill_type: SkillTypes::Arms,
            hits: SkillHits::One,
            damage: [220, 255, 675, 100],
            unlocks: [77, 80, 180, 286],
            hashtags: [
                SkillHashtags::UpperArms,
                SkillHashtags::Posture,
                SkillHashtags::Shoulders,
            ],
            recharge_time: [2, 2, 3, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Tricep Kickback".into(),
            skill_type: SkillTypes::Arms,
            hits: SkillHits::Three,
            damage: [145, 240, 430, 745],
            unlocks: [62, 100, 195, 286],
            hashtags: [
                SkillHashtags::UpperArms,
                SkillHashtags::Empty,
                SkillHashtags::Empty,
            ],
            recharge_time: [2, 3, 4, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Bow Pull".into(),
            skill_type: SkillTypes::Arms,
            hits: SkillHits::Five,
            damage: [35, 210, 370, 655],
            unlocks: [17, 107, 156, 286],
            hashtags: [
                SkillHashtags::UpperArms,
                SkillHashtags::Trapezius,
                SkillHashtags::Core,
            ],
            recharge_time: [2, 3, 4, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Shoulder Press".into(),
            skill_type: SkillTypes::Arms,
            hits: SkillHits::Heal,
            damage: [6, 12, 14, 20],
            unlocks: [52, 119, 156, 286],
            hashtags: [
                SkillHashtags::UpperArms,
                SkillHashtags::Posture,
                SkillHashtags::Shoulders,
            ],
            recharge_time: [3, 3, 4, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Overhead Arm Spin".into(),
            skill_type: SkillTypes::Arms,
            hits: SkillHits::Five,
            damage: [90, 295, 490, 655],
            unlocks: [47, 131, 267, 286],
            hashtags: [
                SkillHashtags::UpperArms,
                SkillHashtags::Shoulders,
                SkillHashtags::Posture,
            ],
            recharge_time: [3, 3, 5, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Overhead Arm Twist".into(),
            skill_type: SkillTypes::Arms,
            hits: SkillHits::One,
            damage: [90, 350, 705, 1000],
            unlocks: [29, 125, 188, 286],
            hashtags: [
                SkillHashtags::UpperArms,
                SkillHashtags::Shoulders,
                SkillHashtags::Core,
            ],
            recharge_time: [2, 2, 4, 0],
            goal_reps: 5000,
            completed_reps: 0,
        },
        Skill {
            name: "Plank".into(),
            skill_type: SkillTypes::Core,
            hits: SkillHits::Three,
            damage: [50, 325, 485, 745],
            unlocks: [20, 132, 172, 286],
            hashtags: [
                SkillHashtags::Abs,
                SkillHashtags::Core,
                SkillHashtags::Posture,
            ],
            recharge_time: [2, 3, 4, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Leg Raise".into(),
            skill_type: SkillTypes::Core,
            hits: SkillHits::One,
            damage: [175, 300, 755, 1000],
            unlocks: [56, 92, 196, 286],
            hashtags: [
                SkillHashtags::Abs,
                SkillHashtags::Core,
                SkillHashtags::Empty,
            ],
            recharge_time: [2, 2, 4, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Open & Close Leg Raise".into(),
            skill_type: SkillTypes::Core,
            hits: SkillHits::Heal,
            damage: [5, 13, 17, 20],
            unlocks: [28, 125, 184, 286],
            hashtags: [
                SkillHashtags::Abs,
                SkillHashtags::Legs,
                SkillHashtags::Glutes,
            ],
            recharge_time: [3, 3, 4, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Overhead Side Bend".into(),
            skill_type: SkillTypes::Core,
            hits: SkillHits::Heal,
            damage: [7, 11, 14, 20],
            unlocks: [65, 119, 146, 286],
            hashtags: [
                SkillHashtags::Waist,
                SkillHashtags::Core,
                SkillHashtags::UpperArms,
            ],
            recharge_time: [3, 3, 4, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Pendulum Bend".into(),
            skill_type: SkillTypes::Core,
            hits: SkillHits::Three,
            damage: [130, 215, 560, 745],
            unlocks: [58, 89, 245, 286],
            hashtags: [
                SkillHashtags::Waist,
                SkillHashtags::LowerBody,
                SkillHashtags::Core,
            ],
            recharge_time: [2, 3, 5, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Overhead Bend".into(),
            skill_type: SkillTypes::Core,
            hits: SkillHits::One,
            damage: [80, 390, 795, 1000],
            unlocks: [20, 116, 204, 286],
            hashtags: [
                SkillHashtags::Core,
                SkillHashtags::Posture,
                SkillHashtags::Trapezius,
            ],
            recharge_time: [1, 2, 4, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Seated Forward Press".into(),
            skill_type: SkillTypes::Core,
            hits: SkillHits::Heal,
            damage: [5, 10, 15, 20],
            unlocks: [37, 95, 159, 286],
            hashtags: [
                SkillHashtags::UpperArms,
                SkillHashtags::Abs,
                SkillHashtags::Flexibility,
            ],
            recharge_time: [3, 3, 4, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Knee-to-Chest".into(),
            skill_type: SkillTypes::Core,
            hits: SkillHits::One,
            damage: [30, 235, 700, 1000],
            unlocks: [1, 74, 226, 286],
            hashtags: [
                SkillHashtags::Abs,
                SkillHashtags::UpperArms,
                SkillHashtags::Core,
            ],
            recharge_time: [1, 2, 3, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Overhead Lunge Twist".into(),
            skill_type: SkillTypes::Core,
            hits: SkillHits::One,
            damage: [155, 360, 840, 1000],
            unlocks: [50, 113, 212, 286],
            hashtags: [
                SkillHashtags::Waist,
                SkillHashtags::Legs,
                SkillHashtags::Core,
            ],
            recharge_time: [2, 2, 4, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Leg Scissors".into(),
            skill_type: SkillTypes::Core,
            hits: SkillHits::Three,
            damage: [135, 280, 445, 745],
            unlocks: [58, 110, 164, 286],
            hashtags: [
                SkillHashtags::Abs,
                SkillHashtags::Legs,
                SkillHashtags::Stamina,
            ],
            recharge_time: [2, 3, 4, 0],
            goal_reps: 5000,
            completed_reps: 0,
        },
        Skill {
            name: "Flutter Kick".into(),
            skill_type: SkillTypes::Core,
            hits: SkillHits::One,
            damage: [175, 470, 625, 1000],
            unlocks: [56, 122, 169, 286],
            hashtags: [
                SkillHashtags::Abs,
                SkillHashtags::Legs,
                SkillHashtags::Empty,
            ],
            recharge_time: [2, 2, 3, 0],
            goal_reps: 5000,
            completed_reps: 0,
        },
        Skill {
            name: "Seated Ring Raise".into(),
            skill_type: SkillTypes::Core,
            hits: SkillHits::One,
            damage: [220, 335, 545, 1000],
            unlocks: [74, 101, 152, 286],
            hashtags: [SkillHashtags::Abs, SkillHashtags::Legs, SkillHashtags::Core],
            recharge_time: [2, 2, 3, 0],
            goal_reps: 5000,
            completed_reps: 0,
        },
        Skill {
            name: "Russian Twist".into(),
            skill_type: SkillTypes::Core,
            hits: SkillHits::Five,
            damage: [130, 235, 455, 655],
            unlocks: [61, 103, 233, 286],
            hashtags: [
                SkillHashtags::Waist,
                SkillHashtags::Abs,
                SkillHashtags::Core,
            ],
            recharge_time: [3, 3, 4, 0],
            goal_reps: 5000,
            completed_reps: 0,
        },
        Skill {
            name: "Standing Twist".into(),
            skill_type: SkillTypes::Core,
            hits: SkillHits::Five,
            damage: [20, 205, 325, 655],
            unlocks: [8, 101, 144, 286],
            hashtags: [
                SkillHashtags::Waist,
                SkillHashtags::Stamina,
                SkillHashtags::Empty,
            ],
            recharge_time: [2, 3, 4, 0],
            goal_reps: 5000,
            completed_reps: 0,
        },
        Skill {
            name: "Overhead Hip Shake".into(),
            skill_type: SkillTypes::Core,
            hits: SkillHits::Five,
            damage: [70, 275, 395, 655],
            unlocks: [38, 122, 177, 286],
            hashtags: [
                SkillHashtags::Waist,
                SkillHashtags::Stamina,
                SkillHashtags::UpperArms,
            ],
            recharge_time: [3, 3, 4, 0],
            goal_reps: 5000,
            completed_reps: 0,
        },
        Skill {
            name: "Squat".into(),
            skill_type: SkillTypes::Legs,
            hits: SkillHits::One,
            damage: [30, 360, 655, 1000],
            unlocks: [1, 116, 215, 286],
            hashtags: [
                SkillHashtags::Legs,
                SkillHashtags::Glutes,
                SkillHashtags::Stamina,
            ],
            recharge_time: [1, 2, 3, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Wide Squat".into(),
            skill_type: SkillTypes::Legs,
            hits: SkillHits::Three,
            damage: [85, 185, 560, 745],
            unlocks: [35, 77, 250, 286],
            hashtags: [
                SkillHashtags::Legs,
                SkillHashtags::Glutes,
                SkillHashtags::Stamina,
            ],
            recharge_time: [2, 3, 5, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Overhead Squat".into(),
            skill_type: SkillTypes::Legs,
            hits: SkillHits::Five,
            damage: [110, 210, 325, 655],
            unlocks: [50, 98, 139, 286],
            hashtags: [
                SkillHashtags::Legs,
                SkillHashtags::Glutes,
                SkillHashtags::Stamina,
            ],
            recharge_time: [3, 3, 3, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Thigh Press".into(),
            skill_type: SkillTypes::Legs,
            hits: SkillHits::One,
            damage: [80, 295, 615, 1000],
            unlocks: [23, 89, 168, 286],
            hashtags: [
                SkillHashtags::Legs,
                SkillHashtags::LowerBody,
                SkillHashtags::Posture,
            ],
            recharge_time: [1, 2, 3, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Hip Lift".into(),
            skill_type: SkillTypes::Legs,
            hits: SkillHits::Heal,
            damage: [6, 11, 16, 20],
            unlocks: [44, 107, 209, 286],
            hashtags: [
                SkillHashtags::Legs,
                SkillHashtags::Glutes,
                SkillHashtags::Core,
            ],
            recharge_time: [3, 3, 4, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Mountain Climber".into(),
            skill_type: SkillTypes::Legs,
            hits: SkillHits::Five,
            damage: [120, 285, 510, 655],
            unlocks: [59, 151, 200, 286],
            hashtags: [
                SkillHashtags::Legs,
                SkillHashtags::UpperArms,
                SkillHashtags::Glutes,
            ],
            recharge_time: [3, 3, 4, 0],
            goal_reps: 3000,
            completed_reps: 0,
        },
        Skill {
            name: "Knee Lift".into(),
            skill_type: SkillTypes::Legs,
            hits: SkillHits::One,
            damage: [50, 275, 615, 1000],
            unlocks: [11, 86, 169, 286],
            hashtags: [
                SkillHashtags::Abs,
                SkillHashtags::Legs,
                SkillHashtags::Stamina,
            ],
            recharge_time: [1, 2, 3, 0],
            goal_reps: 5000,
            completed_reps: 0,
        },
        Skill {
            name: "Side Step".into(),
            skill_type: SkillTypes::Legs,
            hits: SkillHits::Three,
            damage: [160, 295, 545, 725],
            unlocks: [66, 116, 192, 286],
            hashtags: [
                SkillHashtags::UpperArms,
                SkillHashtags::Legs,
                SkillHashtags::Stamina,
            ],
            recharge_time: [2, 3, 4, 0],
            goal_reps: 5000,
            completed_reps: 0,
        },
        Skill {
            name: "Ring Raise Combo".into(),
            skill_type: SkillTypes::Legs,
            hits: SkillHits::One,
            damage: [155, 415, 615, 1000],
            unlocks: [44, 122, 165, 286],
            hashtags: [
                SkillHashtags::Legs,
                SkillHashtags::Glutes,
                SkillHashtags::Stamina,
            ],
            recharge_time: [2, 2, 3, 0],
            goal_reps: 5000,
            completed_reps: 0,
        },
        Skill {
            name: "Knee-Lift Combo".into(),
            skill_type: SkillTypes::Legs,
            hits: SkillHits::Three,
            damage: [165, 240, 490, 745],
            unlocks: [71, 110, 180, 286],
            hashtags: [
                SkillHashtags::Legs,
                SkillHashtags::Glutes,
                SkillHashtags::Stamina,
            ],
            recharge_time: [3, 3, 4, 0],
            goal_reps: 5000,
            completed_reps: 0,
        },
        Skill {
            name: "Chair Pose".into(),
            skill_type: SkillTypes::Yoga,
            hits: SkillHits::One,
            damage: [30, 260, 655, 1000],
            unlocks: [1, 77, 240, 286],
            hashtags: [
                SkillHashtags::LowerBody,
                SkillHashtags::Core,
                SkillHashtags::Stamina,
            ],
            recharge_time: [1, 2, 3, 0],
            goal_reps: 2000,
            completed_reps: 0,
        },
        Skill {
            name: "Boat Pose".into(),
            skill_type: SkillTypes::Yoga,
            hits: SkillHits::Five,
            damage: [155, 320, 495, 655],
            unlocks: [71, 137, 255, 286],
            hashtags: [
                SkillHashtags::Abs,
                SkillHashtags::Core,
                SkillHashtags::Stamina,
            ],
            recharge_time: [3, 3, 5, 0],
            goal_reps: 2000,
            completed_reps: 0,
        },
        Skill {
            name: "Standing Forward Fold".into(),
            skill_type: SkillTypes::Yoga,
            hits: SkillHits::Heal,
            damage: [8, 11, 19, 20],
            unlocks: [70, 113, 208, 286],
            hashtags: [
                SkillHashtags::UpperArms,
                SkillHashtags::Shoulders,
                SkillHashtags::Flexibility,
            ],
            recharge_time: [3, 3, 5, 0],
            goal_reps: 2000,
            completed_reps: 0,
        },
        Skill {
            name: "Tree Pose".into(),
            skill_type: SkillTypes::Yoga,
            hits: SkillHits::One,
            damage: [220, 425, 490, 1000],
            unlocks: [68, 138, 140, 286],
            hashtags: [
                SkillHashtags::Legs,
                SkillHashtags::LowerBody,
                SkillHashtags::Posture,
            ],
            recharge_time: [2, 2, 3, 0],
            goal_reps: 2000,
            completed_reps: 0,
        },
        Skill {
            name: "Hinge Pose".into(),
            skill_type: SkillTypes::Yoga,
            hits: SkillHits::Three,
            damage: [125, 350, 460, 745],
            unlocks: [53, 137, 188, 286],
            hashtags: [
                SkillHashtags::Shoulders,
                SkillHashtags::Legs,
                SkillHashtags::Back,
            ],
            recharge_time: [2, 3, 4, 0],
            goal_reps: 2000,
            completed_reps: 0,
        },
        Skill {
            name: "Revolved Crescent Lunge Pose".into(),
            skill_type: SkillTypes::Yoga,
            hits: SkillHits::One,
            damage: [130, 295, 580, 1000],
            unlocks: [41, 84, 160, 286],
            hashtags: [
                SkillHashtags::Waist,
                SkillHashtags::LowerBody,
                SkillHashtags::Core,
            ],
            recharge_time: [2, 2, 3, 0],
            goal_reps: 2000,
            completed_reps: 0,
        },
        Skill {
            name: "Fan Pose".into(),
            skill_type: SkillTypes::Yoga,
            hits: SkillHits::Heal,
            damage: [4, 9, 15, 20],
            unlocks: [26, 83, 185, 286],
            hashtags: [
                SkillHashtags::Waist,
                SkillHashtags::Flexibility,
                SkillHashtags::Shoulders,
            ],
            recharge_time: [3, 3, 4, 0],
            goal_reps: 2000,
            completed_reps: 0,
        },
        Skill {
            name: "Warrior I Pose".into(),
            skill_type: SkillTypes::Yoga,
            hits: SkillHits::One,
            damage: [60, 300, 580, 1000],
            unlocks: [14, 92, 155, 286],
            hashtags: [
                SkillHashtags::LowerBody,
                SkillHashtags::Aerobic,
                SkillHashtags::Posture,
            ],
            recharge_time: [1, 2, 3, 0],
            goal_reps: 2000,
            completed_reps: 0,
        },
        Skill {
            name: "Warrior II Pose".into(),
            skill_type: SkillTypes::Yoga,
            hits: SkillHits::Five,
            damage: [60, 210, 430, 655],
            unlocks: [32, 95, 176, 286],
            hashtags: [
                SkillHashtags::Chest,
                SkillHashtags::UpperArms,
                SkillHashtags::Shoulders,
            ],
            recharge_time: [2, 3, 4, 0],
            goal_reps: 2000,
            completed_reps: 0,
        },
        Skill {
            name: "Warrior III Pose".into(),
            skill_type: SkillTypes::Yoga,
            hits: SkillHits::Three,
            damage: [125, 330, 440, 745],
            unlocks: [44, 128, 162, 286],
            hashtags: [
                SkillHashtags::Aerobic,
                SkillHashtags::Core,
                SkillHashtags::Stamina,
            ],
            recharge_time: [2, 3, 4, 0],
            goal_reps: 2000,
            completed_reps: 0,
        },
    ]
    .into()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hashtag_string_conv() {
        let glutes_string = "#Glutes";
        let lower_body_string = "#Lower Body";
        let invalid_string = "Something else";

        assert_eq!(
            SkillHashtags::from_str(glutes_string),
            Ok(SkillHashtags::Glutes)
        );
        assert_eq!(
            SkillHashtags::from_str(lower_body_string),
            Ok(SkillHashtags::LowerBody)
        );
        assert_eq!(SkillHashtags::from_str(invalid_string), Err(()));

        assert_eq!(SkillHashtags::Glutes.to_string(), glutes_string);
    }

    #[test]
    fn test_type_string_conv() {
        let arms_string = "Arms";
        let invalid_string = "Something else";

        assert_eq!(SkillTypes::from_str(arms_string), Ok(SkillTypes::Arms));
        assert_eq!(SkillTypes::from_str(invalid_string), Err(()));
        assert_eq!(SkillTypes::Arms.to_string(), arms_string);
    }

    #[test]
    fn test_hit_string_conv() {
        let five_string = "Five";
        let invalid_string = "Something else";

        assert_eq!(SkillHits::from_str(five_string), Ok(SkillHits::Five));
        assert_eq!(SkillHits::from_str(invalid_string), Err(()));
        assert_eq!(SkillHits::Five.to_string(), five_string);
    }

    #[test]
    fn test_skill_equality() {
        let s_one = Skill {
            name: "Test Skill".into(),
            skill_type: SkillTypes::Arms,
            hits: SkillHits::One,
            damage: [0, 0, 0, 0],
            unlocks: [0, 0, 0, 0],
            hashtags: [
                SkillHashtags::Empty,
                SkillHashtags::Empty,
                SkillHashtags::Empty,
            ],
            recharge_time: [0, 0, 0, 0],
            goal_reps: 1000,
            completed_reps: 10,
        };

        let s_two = Skill {
            name: "Test Skill".into(),
            skill_type: SkillTypes::Arms,
            hits: SkillHits::One,
            damage: [0, 0, 0, 0],
            unlocks: [0, 0, 0, 0],
            hashtags: [
                SkillHashtags::Empty,
                SkillHashtags::LowerBody,
                SkillHashtags::Empty,
            ],
            recharge_time: [0, 2, 0, 0],
            goal_reps: 50,
            completed_reps: 99,
        };

        let s_three = Skill {
            name: "Test Skill 2".into(),
            skill_type: SkillTypes::Arms,
            hits: SkillHits::One,
            damage: [0, 0, 0, 0],
            unlocks: [0, 0, 0, 0],
            hashtags: [
                SkillHashtags::Empty,
                SkillHashtags::LowerBody,
                SkillHashtags::Empty,
            ],
            recharge_time: [0, 2, 0, 0],
            goal_reps: 50,
            completed_reps: 99,
        };

        assert_eq!(s_one, s_two);
        assert_ne!(s_two, s_three);
    }

    #[test]
    fn test_skill_attributes() {
        let mut s = Skill {
            name: "Test Skill".into(),
            skill_type: SkillTypes::Arms,
            hits: SkillHits::One,
            damage: [0, 0, 0, 0],
            unlocks: [0, 0, 0, 0],
            hashtags: [
                SkillHashtags::Empty,
                SkillHashtags::Empty,
                SkillHashtags::Empty,
            ],
            recharge_time: [0, 0, 0, 0],
            goal_reps: 1000,
            completed_reps: 10,
        };

        assert!((s.get_rep_percent() - 1.0).abs() < f64::EPSILON);
        assert_eq!(s.get_reps_until_goal(), 990);

        s.completed_reps = 500;

        assert!((s.get_rep_percent() - 50.0).abs() < f64::EPSILON);
        assert_eq!(s.get_reps_until_goal(), 500);

        s.completed_reps = 5000;

        assert!((s.get_rep_percent() - 100.0).abs() < f64::EPSILON);
        assert!((s.get_rep_percent_uncapped() - 500.0).abs() < f64::EPSILON);
        assert_eq!(s.get_reps_until_goal(), 0);
    }
}
