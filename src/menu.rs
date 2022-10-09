use chrono::{Datelike, Timelike};
use egui::{
    CentralPanel, Color32, ComboBox, Context, FontId, Grid, Image, Label, ProgressBar, RichText,
    ScrollArea, Window,
};

use crate::{
    app::RingFitApp,
    lang::{switch_language, Languages},
    skills::{Skill, SkillHashtags, SkillHits, SkillTypes},
    workout::{get_workouts_from_db, save_workout_to_db},
};

#[derive(Debug, PartialEq, Eq)]
pub enum Menu {
    LogWorkout(bool),
    ViewProgress,
    ViewWorkouts,
    ViewSkills,
    SetReps(bool),
    LanguageChoice,
}

// Colors of the different skill types.
const ARMS_COLOR: Color32 = Color32::from_rgb(227, 48, 48);
const CORE_COLOR: Color32 = Color32::from_rgb(227, 227, 48);
const LEGS_COLOR: Color32 = Color32::from_rgb(99, 48, 227);
const YOGA_COLOR: Color32 = Color32::from_rgb(48, 227, 137);
// Color of the Back button.
const BACK_COLOR: Color32 = Color32::from_rgb(155, 0, 0);
// Colors of confirm/cancel buttons
const CONFIRM_COLOR: Color32 = Color32::from_rgb(0, 210, 0);
const CANCEL_COLOR: Color32 = Color32::from_rgb(210, 0, 0);
// Header font size, also used for spacing.
const HEADER_SIZE: f32 = 20.;

/// Checking and displaying the correct menu.
pub fn display_menu(rfa: &mut RingFitApp, ctx: &Context) {
    match rfa.menu {
        Some(Menu::LogWorkout(_)) => {
            log_workout(rfa, ctx);
        }
        Some(Menu::ViewProgress) => {
            view_progess(rfa, ctx);
        }
        Some(Menu::SetReps(_)) => {
            set_reps(rfa, ctx);
        }
        Some(Menu::ViewSkills) => {
            view_skills(rfa, ctx);
        }
        Some(Menu::LanguageChoice) => {
            language_choice(rfa, ctx);
        }
        Some(Menu::ViewWorkouts) => {
            view_workouts(rfa, ctx);
        }
        None => {
            main_menu(rfa, ctx);
        }
    }
}

/// The main menu, with all of the buttons for the sub menus.
pub fn main_menu(rfa: &mut RingFitApp, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        ui.label(RichText::new("Ring Fit Adventure Tracker").size(40.));

        ui.add_space(HEADER_SIZE);

        ui.horizontal(|ui| {
            for image in &rfa.images {
                ui.add(
                    Image::new(image.texture_id(ctx), image.size_vec2())
                        // The tint makes the icons look a bit greyed out.
                        .tint(Color32::from_rgb(100, 100, 100)),
                );
            }
        });

        ui.add_space(HEADER_SIZE);

        if ui
            .button(
                rfa.menu_names
                    .get("log_workout")
                    .unwrap_or(&"Save todays workout".to_owned()),
            )
            .clicked()
        {
            rfa.menu = Some(Menu::LogWorkout(false));
        }
        if ui
            .button(
                rfa.menu_names
                    .get("show_progress")
                    .unwrap_or(&"Show progress".to_owned()),
            )
            .clicked()
        {
            rfa.menu = Some(Menu::ViewProgress);
        }
        if ui
            .button(
                rfa.menu_names
                    .get("show_workouts")
                    .unwrap_or(&"Show previous workouts".to_owned()),
            )
            .clicked()
        {
            rfa.menu = Some(Menu::ViewWorkouts);
        }
        if ui
            .button(
                rfa.menu_names
                    .get("skill_info")
                    .unwrap_or(&"Information about skills".to_owned()),
            )
            .clicked()
        {
            rfa.menu = Some(Menu::ViewSkills);
        }
        if ui
            .button(
                rfa.menu_names
                    .get("set_reps")
                    .unwrap_or(&"Set reps manually".to_owned()),
            )
            .clicked()
        {
            rfa.menu = Some(Menu::SetReps(false));
        }
        // This is always english, just in case you misclick to some language you do not speak and want to switch back.
        if ui.button("Change Language").clicked() {
            rfa.menu = Some(Menu::LanguageChoice);
        }
    });
}

pub fn log_workout(rfa: &mut RingFitApp, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        if ui
            .button(
                RichText::new(rfa.menu_names.get("back").unwrap_or(&"Back".to_owned()))
                    .color(BACK_COLOR),
            )
            .clicked()
        {
            rfa.menu = None;
        }
        ui.add_space(HEADER_SIZE);

        ScrollArea::new([true, true]).show(ui, |ui| {
            Grid::new("RFA").striped(true).show(ui, |ui| {
                let default_value = "Invalid".to_owned();

                let headers = vec![
                    rfa.menu_names.get("skill").unwrap_or(&default_value),
                    rfa.menu_names.get("reps").unwrap_or(&default_value),
                    rfa.menu_names
                        .get("todays_workout")
                        .unwrap_or(&default_value),
                ];

                for text in headers {
                    ui.label(RichText::new(text).size(HEADER_SIZE));
                }
                ui.end_row();

                for (i, skill) in rfa.skills.iter().enumerate() {
                    let color = match skill.skill_type {
                        SkillTypes::Arms => ARMS_COLOR,
                        SkillTypes::Core => CORE_COLOR,
                        SkillTypes::Legs => LEGS_COLOR,
                        SkillTypes::Yoga => YOGA_COLOR,
                    };
                    ui.label(
                        RichText::new(rfa.skill_names.get(skill).unwrap_or(&"".into()))
                            .color(color),
                    );
                    ui.label(RichText::new(skill.completed_reps.to_string()).color(color));
                    ui.text_edit_singleline(&mut rfa.input_reps[i])
                        .on_hover_text(
                            rfa.menu_names
                                .get("enter_todays_reps")
                                .unwrap_or(&"Insert today's reps".to_owned()),
                        );

                    ui.end_row();
                }
            });

            ui.add_space(HEADER_SIZE);

            if ui
                .button(
                    rfa.menu_names
                        .get("log_workout")
                        .unwrap_or(&"Save Workout".to_owned()),
                )
                .clicked()
            {
                rfa.menu = Some(Menu::LogWorkout(true));
            }
        });
    });

    if rfa.menu == Some(Menu::LogWorkout(true)) {
        Window::new(
            rfa.menu_names
                .get("confirm_workout")
                .unwrap_or(&"Confirm Workout".to_owned()),
        )
        .collapsible(false)
        .resizable(false)
        .show(ctx, |ui| {
            ui.label(
                RichText::new(
                    rfa.menu_names
                        .get("todays_workout")
                        .unwrap_or(&"Today's Workout".to_owned()),
                )
                .size(HEADER_SIZE),
            );
            ui.add_space(HEADER_SIZE);

            for (i, skill) in rfa.skills.iter().enumerate() {
                // We check if there is an input and if it is a valid integer.
                if !rfa.input_reps[i].is_empty() && rfa.input_reps[i].parse::<usize>().is_ok() {
                    let color = match skill.skill_type {
                        SkillTypes::Arms => ARMS_COLOR,
                        SkillTypes::Core => CORE_COLOR,
                        SkillTypes::Legs => LEGS_COLOR,
                        SkillTypes::Yoga => YOGA_COLOR,
                    };
                    ui.label(
                        RichText::new(format!(
                            "{}: {}",
                            rfa.skill_names.get(skill).unwrap_or(&"".into()),
                            rfa.input_reps[i]
                        ))
                        .color(color),
                    );
                }
            }

            ui.add_space(HEADER_SIZE);

            ui.horizontal(|ui| {
                // If the user confirms the workout, we log the workout.
                if ui
                    .button(
                        RichText::new(
                            rfa.menu_names
                                .get("confirm")
                                .unwrap_or(&"Confirm".to_owned()),
                        )
                        .color(CONFIRM_COLOR),
                    )
                    .clicked()
                {
                    // First we set the reps for each skill.
                    for (i, skill) in rfa.skills.iter().enumerate() {
                        skill
                            .update_reps(
                                &rfa.db_connection,
                                if rfa.input_reps[i].is_empty() {
                                    0
                                } else {
                                    rfa.input_reps[i].parse().unwrap_or(0)
                                },
                            )
                            .expect("Could not set reps in database.");
                    }

                    // Then we save the workout into the database.
                    save_workout_to_db(
                        &rfa.db_connection,
                        rfa.skills.clone(),
                        rfa.input_reps.clone(),
                    )
                    .expect("Could not save workout to database.");

                    // Then we pass the new values into the RingFitApp.
                    let all_skills = Skill::get_all_skills(&rfa.db_connection);

                    rfa.input_reps = vec!["".into(); all_skills.len()];
                    rfa.skills = all_skills;
                    rfa.menu = Some(Menu::LogWorkout(false));
                }

                // If the user cancels we just throw them into the previous screen without changing anything.
                if ui
                    .button(
                        RichText::new(rfa.menu_names.get("cancel").unwrap_or(&"Cancel".to_owned()))
                            .color(CANCEL_COLOR),
                    )
                    .clicked()
                {
                    rfa.menu = Some(Menu::LogWorkout(false));
                }
            });
        });
    }
}

#[allow(clippy::redundant_closure_for_method_calls)]
pub fn view_progess(rfa: &mut RingFitApp, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        if ui
            .button(
                RichText::new(rfa.menu_names.get("back").unwrap_or(&"Back".to_owned()))
                    .color(BACK_COLOR),
            )
            .clicked()
        {
            rfa.menu = None;
        }
        ui.add_space(HEADER_SIZE);

        ScrollArea::new([true, true]).show(ui, |ui| {
            Grid::new("progress").show(ui, |ui| {
                let default_value = "Invalid".to_owned();

                let headers = vec![
                    rfa.menu_names.get("skill").unwrap_or(&default_value),
                    rfa.menu_names.get("reps").unwrap_or(&default_value),
                    rfa.menu_names.get("pending").unwrap_or(&default_value),
                    rfa.menu_names
                        .get("progress_percent")
                        .unwrap_or(&default_value),
                ];

                for text in headers {
                    ui.label(RichText::new(text).size(HEADER_SIZE));
                }
                ui.end_row();

                for skill in &rfa.skills {
                    ui.label(rfa.skill_names.get(skill).unwrap_or(&"".into()));
                    let color = match skill.get_rep_percent_uncapped() {
                        x if x >= 200.0 => Color32::from_rgb(42, 92, 9),
                        x if x >= 150.0 => Color32::from_rgb(69, 153, 15),
                        x if x >= 100.0 => Color32::from_rgb(90, 201, 20),
                        x if x >= 75.0 => Color32::from_rgb(199, 153, 26),
                        x if x >= 50.0 => Color32::from_rgb(199, 101, 26),
                        x if x >= 25.0 => Color32::from_rgb(158, 21, 21),
                        _ => Color32::from_rgb(87, 16, 16),
                    };
                    ui.label(RichText::new(skill.completed_reps.to_string()).color(color));
                    ui.label(RichText::new(skill.get_reps_until_goal().to_string()).color(color));
                    ui.add(
                        ProgressBar::new(skill.get_rep_percent() as f32 / 100.0).show_percentage(),
                    )
                    .on_hover_text(format!("{:.5}%", skill.get_rep_percent().to_string()));
                    ui.end_row();
                }

                ui.label(
                    RichText::new(rfa.menu_names.get("total").unwrap_or(&"Total".to_owned()))
                        .strong(),
                );

                let all_sum = rfa.skills.iter().map(|s| s.completed_reps).sum::<usize>();
                let all_goal = rfa
                    .skills
                    .iter()
                    .map(|s| s.get_reps_until_goal())
                    .sum::<usize>();

                ui.label(RichText::new(all_sum.to_string()).strong());
                ui.label(RichText::new(all_goal.to_string()).strong());
                ui.vertical(|ui| {
                    let total_percent = 1.0
                        - (all_goal as f32
                            / rfa.skills.iter().map(|s| s.goal_reps).sum::<usize>() as f32);
                    let relative_percent =
                        ((rfa.skills.iter().map(|s| s.get_rep_percent()).sum::<f64>()
                            / rfa.skills.len() as f64)
                            / 100.0) as f32;

                    ui.add(ProgressBar::new(total_percent).show_percentage())
                        .on_hover_text(format!("{:.5}%", total_percent * 100.0));
                    ui.add(ProgressBar::new(relative_percent).show_percentage())
                        .on_hover_text(format!("{:.5}%", relative_percent * 100.0));
                });
            });
        });
    });
}

pub fn set_reps(rfa: &mut RingFitApp, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        if ui
            .button(
                RichText::new(rfa.menu_names.get("back").unwrap_or(&"Back".to_owned()))
                    .color(BACK_COLOR),
            )
            .clicked()
        {
            rfa.menu = None;
        }
        ui.add_space(HEADER_SIZE);

        ScrollArea::new([true, true]).show(ui, |ui| {
            Grid::new("set_reps").striped(true).show(ui, |ui| {
                let default_value = "Invalid".to_owned();

                let headers = vec![
                    rfa.menu_names.get("skill").unwrap_or(&default_value),
                    rfa.menu_names.get("reps").unwrap_or(&default_value),
                    rfa.menu_names.get("new_reps").unwrap_or(&default_value),
                ];

                for text in headers {
                    ui.label(RichText::new(text).size(HEADER_SIZE));
                }
                ui.end_row();

                for (i, skill) in rfa.skills.iter().enumerate() {
                    let color = match skill.skill_type {
                        SkillTypes::Arms => ARMS_COLOR,
                        SkillTypes::Core => CORE_COLOR,
                        SkillTypes::Legs => LEGS_COLOR,
                        SkillTypes::Yoga => YOGA_COLOR,
                    };
                    ui.label(
                        RichText::new(rfa.skill_names.get(skill).unwrap_or(&"".to_owned()))
                            .color(color),
                    );
                    ui.label(RichText::new(skill.completed_reps.to_string()).color(color));
                    ui.text_edit_singleline(&mut rfa.input_reps[i])
                        .on_hover_text(
                            rfa.menu_names
                                .get("enter_total_reps")
                                .unwrap_or(&"Please enter the total amount of reps".to_owned()),
                        );

                    ui.end_row();
                }
            });

            ui.add_space(HEADER_SIZE);

            if ui
                .button(
                    rfa.menu_names
                        .get("save_reps")
                        .unwrap_or(&"Save reps".to_owned()),
                )
                .clicked()
            {
                rfa.menu = Some(Menu::SetReps(true));
            }
        });

        if rfa.menu == Some(Menu::SetReps(true)) {
            Window::new(
                rfa.menu_names
                    .get("confirm_reps")
                    .unwrap_or(&"Confirm Reps".to_owned()),
            )
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label(
                    RichText::new(
                        rfa.menu_names
                            .get("show_set_reps")
                            .unwrap_or(&"Set these reps?".to_owned()),
                    )
                    .size(HEADER_SIZE),
                );
                ui.add_space(HEADER_SIZE);

                for (i, skill) in rfa.skills.iter().enumerate() {
                    if !rfa.input_reps[i].is_empty() && rfa.input_reps[i].parse::<usize>().is_ok() {
                        let color = match skill.skill_type {
                            SkillTypes::Arms => ARMS_COLOR,
                            SkillTypes::Core => CORE_COLOR,
                            SkillTypes::Legs => LEGS_COLOR,
                            SkillTypes::Yoga => YOGA_COLOR,
                        };
                        ui.label(
                            RichText::new(format!(
                                "{}: {} ➡ {}",
                                rfa.skill_names.get(skill).unwrap_or(&"".into()),
                                skill.completed_reps,
                                rfa.input_reps[i]
                            ))
                            .color(color),
                        );
                    }
                }
                ui.add_space(HEADER_SIZE);

                ui.horizontal(|ui| {
                    if ui
                        .button(
                            RichText::new(
                                rfa.menu_names
                                    .get("confirm")
                                    .unwrap_or(&"Confirm".to_owned()),
                            )
                            .color(CONFIRM_COLOR),
                        )
                        .clicked()
                    {
                        for (i, skill) in rfa.skills.iter().enumerate() {
                            if !rfa.input_reps[i].is_empty()
                                && rfa.input_reps[i].parse::<usize>().is_ok()
                            {
                                skill
                                    .set_reps(
                                        &rfa.db_connection,
                                        rfa.input_reps[i].parse().unwrap_or(0),
                                    )
                                    .expect("Could not set reps in database.");
                            }
                        }

                        let all_skills = Skill::get_all_skills(&rfa.db_connection);

                        rfa.input_reps = vec!["".into(); all_skills.len()];
                        rfa.skills = all_skills;
                        rfa.menu = Some(Menu::SetReps(false));
                    }

                    if ui
                        .button(
                            RichText::new(
                                rfa.menu_names.get("cancel").unwrap_or(&"Cancel".to_owned()),
                            )
                            .color(CANCEL_COLOR),
                        )
                        .clicked()
                    {
                        rfa.menu = Some(Menu::SetReps(false));
                    }
                });
            });
        }
    });
}

pub fn view_skills(rfa: &mut RingFitApp, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        if ui
            .button(
                RichText::new(rfa.menu_names.get("back").unwrap_or(&"Back".to_owned()))
                    .color(BACK_COLOR),
            )
            .clicked()
        {
            rfa.menu = None;
        }
        ui.add_space(HEADER_SIZE);

        ScrollArea::new([true, true]).show(ui, |ui| {
            Grid::new("view_skills").striped(true).show(ui, |ui| {
                let default_name = "Invalid".to_owned();

                let headers = vec![
                    rfa.menu_names.get("name").unwrap_or(&default_name),
                    rfa.menu_names.get("hits").unwrap_or(&default_name),
                    rfa.menu_names.get("level").unwrap_or(&default_name),
                    rfa.menu_names.get("damage").unwrap_or(&default_name),
                    rfa.menu_names.get("unlocks").unwrap_or(&default_name),
                    rfa.menu_names.get("cooldown").unwrap_or(&default_name),
                    rfa.menu_names.get("hashtags").unwrap_or(&default_name),
                ];

                for text in headers {
                    ui.label(RichText::new(text).size(HEADER_SIZE));
                }
                ui.end_row();

                for (i, skill) in rfa.skills.iter().enumerate() {
                    ui.label(
                        RichText::new(format!(
                            "{}) {}",
                            i + 1,
                            rfa.skill_names.get(skill).unwrap_or(&default_name)
                        ))
                        .color(match skill.skill_type {
                            SkillTypes::Arms => ARMS_COLOR,
                            SkillTypes::Core => CORE_COLOR,
                            SkillTypes::Legs => LEGS_COLOR,
                            SkillTypes::Yoga => YOGA_COLOR,
                        }),
                    );
                    ui.label(
                        RichText::new(match skill.hits {
                            SkillHits::One => "    🎯    ",
                            SkillHits::Three => "  🎯🎯🎯  ",
                            SkillHits::Five => "🎯🎯🎯🎯🎯",
                            SkillHits::Heal => "    ❤    ",
                        })
                        .font(FontId::monospace(14.)),
                    );

                    ui.vertical(|ui| {
                        for num in [1, 2, 3, 4] {
                            ui.label(format!("{}", num));
                        }
                    });

                    ui.vertical(|ui| {
                        for dmg in skill.damage {
                            ui.label(dmg.to_string());
                        }
                    });

                    ui.vertical(|ui| {
                        for unlock in skill.unlocks {
                            ui.label(unlock.to_string());
                        }
                    });

                    ui.vertical(|ui| {
                        for time in skill.recharge_time {
                            ui.label(time.to_string());
                        }
                    });

                    ui.vertical(|ui| {
                        for hashtag in &skill.hashtags {
                            ui.add(
                                Label::new(RichText::new(match hashtag {
                                    SkillHashtags::Empty => "",
                                    _ => rfa.hashtag_names.get(hashtag).unwrap_or(&default_name),
                                }))
                                .wrap(false),
                            );
                        }
                    });

                    ui.end_row();
                }
            });
        });
    });
}

pub fn language_choice(rfa: &mut RingFitApp, ctx: &Context) {
    CentralPanel::default().show(ctx, |ui| {
        if ui
            .button(
                RichText::new(rfa.menu_names.get("back").unwrap_or(&"Back".to_owned()))
                    .color(BACK_COLOR),
            )
            .clicked()
        {
            rfa.menu = None;
        }
        ui.add_space(HEADER_SIZE);

        ComboBox::from_label(
            rfa.menu_names
                .get("lang_select")
                .unwrap_or(&"Select a language".to_owned()),
        )
        .selected_text(format!("{:?}", rfa.language))
        .show_ui(ui, |ui| {
            if ui
                .selectable_value(&mut rfa.language, Languages::English, "English")
                .clicked()
            {
                switch_language(rfa, rfa.language);
            };
            if ui
                .selectable_value(&mut rfa.language, Languages::German, "Deutsch")
                .clicked()
            {
                switch_language(rfa, rfa.language);
            };
        });
    });
}

pub fn view_workouts(rfa: &mut RingFitApp, ctx: &Context) {
    let workouts = get_workouts_from_db(&rfa.db_connection);

    CentralPanel::default().show(ctx, |ui| {
        if ui
            .button(
                RichText::new(rfa.menu_names.get("back").unwrap_or(&"Back".to_owned()))
                    .color(BACK_COLOR),
            )
            .clicked()
        {
            rfa.menu = None;
        }
        ui.add_space(HEADER_SIZE);

        ScrollArea::new([true, true]).show(ui, |ui| {
            Grid::new("view_skills").striped(true).show(ui, |ui| {
                let default_name = "Invalid".to_owned();

                let headers = vec![
                    rfa.menu_names.get("time").unwrap_or(&default_name),
                    rfa.menu_names.get("skill").unwrap_or(&default_name),
                    rfa.menu_names.get("reps").unwrap_or(&default_name),
                ];

                for text in headers {
                    ui.label(RichText::new(text).size(HEADER_SIZE));
                }
                ui.end_row();

                for (time, workout) in workouts {
                    ui.label(format!(
                        "{}/{:02}/{:02} - {:02}:{:02}",
                        time.year(),
                        time.month(),
                        time.day(),
                        time.hour(),
                        time.minute()
                    ));

                    ui.vertical(|ui| {
                        for (skill, _) in &workout.skill {
                            let color = match skill.skill_type {
                                SkillTypes::Arms => ARMS_COLOR,
                                SkillTypes::Core => CORE_COLOR,
                                SkillTypes::Legs => LEGS_COLOR,
                                SkillTypes::Yoga => YOGA_COLOR,
                            };
                            ui.add(
                                Label::new(
                                    RichText::new(
                                        rfa.skill_names.get(skill).unwrap_or(&"".to_owned()),
                                    )
                                    .color(color),
                                )
                                .wrap(false),
                            );
                        }
                    });

                    ui.vertical(|ui| {
                        for (skill, reps) in &workout.skill {
                            let color = match skill.skill_type {
                                SkillTypes::Arms => ARMS_COLOR,
                                SkillTypes::Core => CORE_COLOR,
                                SkillTypes::Legs => LEGS_COLOR,
                                SkillTypes::Yoga => YOGA_COLOR,
                            };
                            ui.add(
                                Label::new(RichText::new(reps.to_string()).color(color))
                                    .wrap(false),
                            );
                        }
                    });

                    ui.end_row();
                    ui.separator();
                    ui.end_row();
                }
            });
        });
    });
}
