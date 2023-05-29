use crate::config;
use crate::parse;
use crate::utils::*;

use eframe::egui;
use std::collections::HashMap;

pub fn gui_create() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Running Workout",
        native_options,
        Box::new(|cc| Box::new(WorkoutApp::new(cc))),
    )
}

#[derive(Default)]
struct WorkoutApp {
    config: HashMap<String, f32>,
    workout: String,
}

impl WorkoutApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            config: HashMap::from([
                ("E".to_owned(), pace2speed("5:40")),
                ("M".to_owned(), pace2speed("5:00")),
                ("T".to_owned(), pace2speed("4:30")),
                ("I".to_owned(), pace2speed("4:00")),
                ("H".to_owned(), pace2speed("4:00")),
                ("R".to_owned(), pace2speed("3:30")),
                ("jg".to_owned(), pace2speed("8:00")),
                ("jog".to_owned(), pace2speed("8:00")),
                ("rst".to_owned(), pace2speed("15:00")),
            ]),
            workout: "5 E + 3 * (1 I + 2 min rst) + 3 E".to_owned(),
        }
    }
}

fn paces_to_strings(input: &HashMap<String, f32>) -> HashMap<String, String> {
    let mut out = HashMap::new();
    for (k, v) in input.iter() {
        out.insert(k.clone(), speed2pace(*v));
    }
    out
}

impl eframe::App for WorkoutApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Paces");
            for (k, v) in self.config.iter_mut() {
                ui.add(
                    egui::Slider::new(v, 1.0..=8.0)
                        .text(k)
                        .custom_formatter(|n, _| speed2pace(n as f32))
                        .custom_parser(|s| Some(f64::from(pace2speed(s)))),
                );
            }
            ui.heading("Workout");
            ui.text_edit_singleline(&mut self.workout);

            ui.heading("Summary");
            config::init(paces_to_strings(&self.config));
            ui.label(format!("{}", parse::summarize(&self.workout)));
        });
    }
}
