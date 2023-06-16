use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::{Path, PathBuf};

// external crates
use clap::{arg, Parser};

#[allow(dead_code)]
fn write_file(data: &[u8], path: &Path) {
    let mut file = match File::create(path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };
    if let Err(why) = file.write_all(data) {
        panic!("couldn't write to {}: {}", path.display(), why);
    };
}

fn read_workout_file(path: PathBuf) -> Vec<String> {
    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };
    BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // workout definition as positional argument
    #[arg(short, long)]
    workout: Option<String>,

    // workout file as alternative to single workout argument
    #[arg(short, long)]
    file: Option<PathBuf>,

    // config file
    #[arg(short, long)]
    config: Option<PathBuf>,

    // start egui
    #[arg(short, long)]
    gui: bool,

    // verbose level
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    paces: HashMap<String, String>,
}

fn main() {
    let cli = Cli::parse();

    // set RUST_LOG environment variable for logging config
    match cli.verbose {
        0 => { /* do nothing */ }
        1 => env::set_var("RUST_LOG", "info"),
        _ => env::set_var("RUST_LOG", "debug"),
    }
    env_logger::init();

    let c = cli.config.unwrap_or_else(|| PathBuf::from("paces.toml"));
    let mut f = File::open(c).expect("Couldn't open config file");
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    let config: Config = toml::from_str(&s).unwrap();
    workout::init(config.paces);

    if let Some(w) = cli.workout {
        println!(
            "{}",
            workout::summarize(w.as_str()).unwrap_or("invalid workout".to_string())
        );
    }

    if let Some(workouts_file) = cli.file {
        for w in read_workout_file(workouts_file) {
            println!(
                "{}",
                workout::summarize(&w).unwrap_or("invalid workout".to_string())
            );
        }
    }

    if cli.gui {
        workout::gui_create().unwrap();
    }
}
