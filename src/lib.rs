mod parse;
mod wtree;
mod config;

use wtree::DistanceAndTime;

use log::*;


pub fn simple_parse(input: String) {
    let w = parse::parse_Workout(&input).unwrap();
    info!("{}", input);
    info!(
        "({:.*} km, {}:{:02} h)",
        1,
        w.distance() as f32 / 1000.0,
        w.time() as i32 / 3600,
        w.time() as i32 % 3600 / 60
    );
}

