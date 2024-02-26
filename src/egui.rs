use crate::config;
use crate::parse;
use crate::utils::*;

use eframe::egui;
use std::collections::HashMap;

#[cfg(not(target_arch = "wasm32"))]
pub fn gui_create() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Running Workout",
        native_options,
        Box::new(|cc| Box::new(WorkoutApp::new(cc))),
    )
}

#[cfg(target_arch = "wasm32")]
pub fn gui_create() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();
    let web_options = eframe::WebOptions {
        max_size_points: egui::Vec2 { x: 800.0, y: 500.0 },
        ..Default::default()
    };

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "egui_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(WorkoutApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}

#[derive(Default)]
struct WorkoutApp {
    config: HashMap<String, f32>,
    workout: String,
    tmp: Tmp, // used for intermediate, temporary gui data
}

#[derive(Default)]
struct Tmp {
    new_pace: String,
    new_intensity: String,
    remove_config: String,
}

impl WorkoutApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            config: HashMap::from([
                ("E".to_owned(), pace2speed("5:40")),
                ("M".to_owned(), pace2speed("5:00")),
                ("T".to_owned(), pace2speed("4:30")),
                ("I".to_owned(), pace2speed("4:00")),
                ("R".to_owned(), pace2speed("3:30")),
            ]),
            workout: "5 E + 3 * (1 I + 2 min E) + 3 E".to_owned(),
            tmp: Tmp::default(),
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
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Intensities");
                    for (k, v) in self.config.iter_mut() {
                        ui.horizontal(|ui| {
                            if ui.button("🗙").clicked() {
                                self.tmp.remove_config = k.clone(); // schedule for removal
                            }
                            ui.add(
                                egui::Slider::new(v, 1.0..=8.0)
                                    .text(k)
                                    .custom_formatter(|n, _| speed2pace(n as f32))
                                    .custom_parser(|s| Some(f64::from(pace2speed(s))))
                                    .trailing_fill(true),
                            );
                        });
                    }
                });
                ui.vertical(|ui| {
                    ui.heading("Add new intensity");

                    ui.horizontal(|ui| {
                        ui.label("Intensity name:");
                        ui.text_edit_singleline(&mut self.tmp.new_intensity);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Pace (min:sec/km):");
                        ui.text_edit_singleline(&mut self.tmp.new_pace);
                    });
                    if ui.button("➕").clicked() {
                        self.config.insert(
                            self.tmp.new_intensity.clone(),
                            pace2speed(&self.tmp.new_pace),
                        );
                        self.tmp.new_intensity = "".to_owned();
                        self.tmp.new_pace = "".to_owned();
                    }
                });
            });

            ui.vertical(|ui| {
                ui.heading("Workout");
                ui.text_edit_singleline(&mut self.workout);

                ui.heading("Summary");
                config::init(paces_to_strings(&self.config));
                ui.label(parse::summarize(&self.workout).unwrap_or("invalid workout".to_string()));
            });
        });

        // processing
        if self.config.contains_key(&self.tmp.remove_config) {
            self.config.remove(&self.tmp.remove_config);
            self.tmp.remove_config = "".to_owned();
        }
    }
}
