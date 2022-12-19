#![allow(dead_code)]
#![allow(unused_imports)]

use clap::Command;
use workout;

use std::env;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::{Path, PathBuf};

// external crates
use clap::{arg, Parser};
use env_logger;
use log;

fn write_file(data: &[u8], path: &Path) {
    let mut file = match File::create(&path) {
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

    // verbose level
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let cli = Cli::parse();

    // set RUST_LOG environment variable for logging config
    match cli.verbose {
        0 => { /* do nothing */ }
        1 => env::set_var("RUST_LOG", "info"),
        2 | _ => env::set_var("RUST_LOG", "debug"),
    }
    env_logger::init();

    let c = cli.config.unwrap_or(PathBuf::from("paces.toml"));
    workout::init(c.as_path().to_str().unwrap()); // XXX there should be a nicer way

    if let Some(w) = cli.workout {
        println!("{}", workout::summarize(w.as_str()));
    }

    if let Some(workouts_file) = cli.file {
        for w in read_workout_file(workouts_file) {
            println!("{}", workout::summarize(&w));
        }
    }
}
