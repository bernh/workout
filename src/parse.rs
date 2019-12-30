use crate::wtree;
use crate::wtree::{pace2speed, DistanceAndTime};
use crate::config::get_pace;

use log::*;
use nom::{
        IResult,
        bytes::complete::{tag, take_while},
        character::{is_digit},
        sequence::{tuple,terminated},
        branch::{alt},
};

pub fn log_parse(input: &str) {
    let (s, w) = parse_step(&input).unwrap();
    info!("{}", s);
    info!(
        "({:.*} km, {}:{:02} h)",
        1,
        w.distance() as f32 / 1000.0,
        w.time() as i32 / 3600,
        w.time() as i32 % 3600 / 60
    );
}

pub fn parse_workout(workout: &str) -> Result<wtree::Workout, &str> {
    info!("parsing workout: {}", workout);
    todo!();
}

// --- nom parser combinator functions ---

fn is_float_digit(c: char) -> bool {
    c.is_ascii_digit() || c == '.'
} 

pub fn parse_step(input: &str) -> IResult<&str, wtree::Step> {
    info!("parsing step: {}", input);
    // <time run> | <distance run>
    alt((parse_time_run, parse_distance_run))(input)
}

fn parse_distance_run(input: &str) -> IResult<&str, wtree::Step> {
    // <distance> <effort>
    let (input, (distance, effort)) = tuple((parse_distance, parse_effort))(input)?;
    if distance < 100.0 {
        Ok((input, wtree::Step::from_distance(distance * 1000.0, pace2speed(get_pace(&effort)))))
    } else {
        Ok((input, wtree::Step::from_distance(distance, pace2speed(get_pace(&effort)))))
    }
}

fn parse_time_run(input: &str) -> IResult<&str, wtree::Step> {
    // <time [min]> <effort>
    let (input, (time, effort)) = tuple((parse_time, parse_effort))(input)?;
    Ok((input, wtree::Step::from_time(time * 60.0, pace2speed(get_pace(&effort)))))
}

fn parse_distance(input: &str) -> IResult<&str, f32> {

    let (input, distance) = take_while(is_float_digit)(input)?;
    Ok((input, distance.parse::<f32>().unwrap()))

} 

fn parse_time(input: &str) -> IResult<&str, f32> {
    let (input, time) = terminated(take_while(is_float_digit),tag("min"))(input)?;
    Ok((input, time.parse::<f32>().unwrap()))
}

fn parse_effort(input: &str) -> IResult<&str, String> {
    Ok((input, "E".to_string()))
}


// ---------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wtree::DistanceAndTime;

    #[test]
    fn single_step() {
        let (_, w) = parse_step("3E").unwrap();
        assert_eq!(w.distance() as i32, 3000);
        assert_eq!(w.time() as i32, (3 * 6 * 60));
    }
/* 
    #[test]
    fn single_step_workout() {
        let r = parse_workout("3jog");
        let w = r.unwrap();
        assert_eq!(w.nodes.len(), 1);
        assert_approx_eq!(w.distance(), 3000_f32, 0.1);
        assert_approx_eq!(w.time(), (3 * 8 * 60) as f32, 0.1);
    }

    #[test]
    fn multi_step_workout() {
        let r = parse_workout("3M+3T");
        let w = r.unwrap();
        assert_eq!(w.nodes.len(), 2);
        assert_approx_eq!(w.distance(), 6000_f32, 0.1);
        assert_approx_eq!(w.time(), (3 * (5 * 60) + 3 * (4 * 60 + 30)) as f32, 0.1);
    }

    #[test]
    fn repeats() {
        let r = parse_workout("2min I + 3*(1min H + 5min jg)");
        let w = r.unwrap();
        assert_eq!(w.nodes.len(), 2);
        assert_approx_eq!(w.time(), ((2 + 3 * (1 + 5)) * 60) as f32, 0.1);
    } */
}
