use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::Write;
use std::io::BufReader;
use std::io::BufRead;

extern crate workout;
extern crate clap;
use clap::{Arg, App};

fn write_file(data: &[u8], path: &Path) {
    let mut file = match File::create(&path) {
        Err(why) => {
            panic!("couldn't create {}: {}",
                   path.display(),
                   Error::description(&why))
        }
        Ok(file) => file,
    };
    if let Err(why) = file.write_all(data) {
        panic!("couldn't write to {}: {}",
               path.display(),
               Error::description(&why));
    };
}

fn read_workout_file(path: &Path) -> Vec<String> {
    // Open the path in read-only mode, returns `io::Result<File>`
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    BufReader::new(file).lines().map(|x| x.unwrap()).collect()
}


fn main() {
    let matches = App::new("JD Workouts")
                          .version("1.0")
                          .author("Bernhard Leiner <bleiner@gmail.com>")
                          .about("Does awesome things")
                          .arg(Arg::with_name("workouts")
                               .short("w")
                               .long("workouts")
                               .value_name("FILE")
                               .required(true)
                               .help("Sets workout file")
                               .takes_value(true))
                          .get_matches();
    let workouts_file = matches.value_of("workouts").unwrap();

    for w in read_workout_file(Path::new(workouts_file)) {
        workout::simple_parse(w);
    }


    // let mut header = workout::FitFileHeader::new();
    // header.calc_crc();
    // let array = header.bin();
    // println!("{:?}", array);
    // write_file(&array, Path::new("workout.fit"));
}



