use crate::config::get_pace;
use crate::wtree;
use crate::wtree::{pace2speed, DistanceAndTime, Step, Workout};

use log::*;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::is_digit,
    multi::separated_list,
    sequence::{terminated, tuple},
    IResult,
};

pub fn log_parse(input: &str) {
    let (_, w) = parse_workout(&normalize_input(input)).unwrap();
    info!(
        "({:.*} km, {}:{:02} h)",
        1,
        w.distance() as f32 / 1000.0,
        w.time() as i32 / 3600,
        w.time() as i32 % 3600 / 60
    );
}

fn normalize_input(input: &str) -> String {
    let norm : String = input.split_whitespace().collect();
    format!("1*({})", norm)
}

// --- nom parser combinator functions ---
pub fn parse_workout(input: &str) -> IResult<&str, wtree::Workout> {
    // <rep> "*" "("<parts>")"
    let (rem_input, (rep, _, _, parts, _)) = tuple((
        take_while(|c: char| c.is_digit(10)),
        tag("*"),
        tag("("),
        parse_parts,
        tag(")"),
    ))(input)?;
    info!("New Workout from: {}", input);
    let mut w = Workout::new(rep.parse::<i32>().unwrap());
    w.nodes = parts;
    Ok((rem_input, w))
}

pub fn parse_parts(input: &str) -> IResult<&str, Vec<Box<dyn wtree::DistanceAndTime>>> {
    separated_list(tag("+"), parse_part)(input)
}

pub fn parse_part(input: &str) -> IResult<&str, Box<dyn wtree::DistanceAndTime>> {
    // <workout> | <step>
    let res_w = parse_workout(input);
    if let Ok((rem_input, workout)) = res_w {
        return Ok((rem_input, Box::new(workout)));
    }

    let res_s = parse_step(input);
    match res_s {
        Ok((rem_input, step)) => Ok((rem_input, Box::new(step))),
        Err(e) => Err(e),
    }
}

pub fn parse_step(input: &str) -> IResult<&str, wtree::Step> {
    // <time step> | <distance step>
    alt((parse_time_step, parse_distance_step))(input)
}

fn parse_distance_step(input: &str) -> IResult<&str, wtree::Step> {
    // <distance> <effort>
    let (rem_input, (distance, effort)) = tuple((parse_distance, parse_effort))(input)?;
    info!("New distance step from: {}", input);
    if distance < 100.0 {
        Ok((
            rem_input,
            wtree::Step::from_distance(distance * 1000.0, pace2speed(get_pace(effort))),
        ))
    } else {
        Ok((
            rem_input,
            wtree::Step::from_distance(distance, pace2speed(get_pace(effort))),
        ))
    }
}

fn parse_time_step(input: &str) -> IResult<&str, wtree::Step> {
    // <time [min]> <effort>
    let (rem_input, (time, effort)) = tuple((parse_time, parse_effort))(input)?;
    info!("New distance step from: {}", input);
    Ok((
        rem_input,
        wtree::Step::from_time(time, pace2speed(get_pace(effort))),
    ))
}

fn parse_distance(input: &str) -> IResult<&str, f32> {
    let (input, distance) = take_while(is_float_digit)(input)?;
    Ok((input, distance.parse::<f32>().unwrap()))
}

fn parse_time(input: &str) -> IResult<&str, f32> {
    let (rem_input, time) = alt((
        terminated(take_while(is_float_digit), tag("min")),
        terminated(take_while(is_float_digit), tag("s")),
    ))(input)?;
    Ok((
        rem_input,
        time.parse::<f32>().unwrap() * {
            if input.contains("min") {
                60.0
            } else {
                1.0
            }
        },
    ))
}

fn parse_effort(input: &str) -> IResult<&str, &str> {
    // TODO: drop this compile time list and allow everything that has a pace according to the config
    alt((
        tag("E"),
        tag("M"),
        tag("Tempo"),
        tag("T"),
        tag("HM"),
        tag("M"),
        tag("CV"),
        tag("H"),
        tag("I"),
        tag("jg"),
        tag("jog"),
        tag("rst"),
        tag("rest"),
    ))(input)
}

fn is_float_digit(c: char) -> bool {
    c.is_ascii_digit() || c == '.'
}

// ---------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wtree::DistanceAndTime;
    use approx::assert_abs_diff_eq;

    #[test]
    fn single_step_1() {
        let (_, w) = parse_step("3E").unwrap();
        assert_abs_diff_eq!(w.distance(), 3000_f32);
        assert_abs_diff_eq!(w.time(), (3 * 6 * 60) as f32);
    }

    #[test]
    fn single_step_2() {
        let (_, s) = parse_step("360s E").unwrap();
        assert_abs_diff_eq!(s.distance(), 1000_f32, epsilon = 0.1);
        assert_abs_diff_eq!(s.time(), 360_f32);
    }

    #[test]
    fn single_step_workout() {
        let (_, w) = parse_workout(&normalize_input("3jog")).unwrap();
        assert_eq!(w.nodes.len(), 1);
        assert_abs_diff_eq!(w.distance(), 3000_f32, epsilon = 0.1);
        assert_abs_diff_eq!(w.time(), (3 * 8 * 60) as f32, epsilon = 0.1);
    }
    #[test]
    fn multi_step_workout() {
        let (_, w) = parse_workout(&normalize_input("3 M + 3 T")).unwrap();
        assert_eq!(w.nodes.len(), 2);
        assert_abs_diff_eq!(w.distance(), 6000_f32, epsilon = 0.1);
        assert_abs_diff_eq!(
            w.time(),
            (3 * (5 * 60) + 3 * (4 * 60 + 30)) as f32,
            epsilon = 0.1
        );
    }

    #[test]
    fn repeats() {
        let (_, w) = parse_workout(&normalize_input("2min I + 3*(1min H + 5min jg)")).unwrap();
        assert_eq!(w.nodes.len(), 2);
        assert_abs_diff_eq!(w.time(), ((2 + 3 * (1 + 5)) * 60) as f32, epsilon = 0.1);
    }
}
