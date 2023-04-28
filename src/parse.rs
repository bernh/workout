use crate::config::{get_intensities, get_pace};
use crate::wtree;
use crate::wtree::{pace2speed, RunPart};

use log::info;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::digit1,
    error::{Error, ErrorKind},
    multi::separated_list1,
    sequence::tuple,
    Err, IResult,
};

pub fn summarize(input: &str) -> String {
    let (_, w) = parse_workout(&normalize_input(input)).unwrap();
    info!("{}", w);
    format!(
        "{:.*} km, {}:{:02} h, {}:{:02} min/km",
        1,
        w.calc_distance() / 1000.0,
        w.calc_time() as i32 / 3600,
        w.calc_time() as i32 % 3600 / 60,
        (w.calc_time() / (w.calc_distance() / 1000.0)) as i32 / 60,
        (w.calc_time() / (w.calc_distance() / 1000.0)) as i32 % 60,
    )
}

fn normalize_input(input: &str) -> String {
    let norm: String = input.split_whitespace().collect();
    // top level may be list of parts or a repetion. Normalize to always start with a
    // repetition
    format!("1*({})", norm)
}

// --- nom parser combinator functions ---
pub fn parse_workout(input: &str) -> IResult<&str, wtree::RunPart> {
    // <rep> "*" "("<parts>")"
    let (rem_input, (rep, _, _, parts, _)) =
        tuple((digit1, tag("*"), tag("("), parse_parts, tag(")")))(input)?;
    info!("New Workout from: {}", input);
    let mut w = RunPart::new_workout(rep.parse::<i32>().unwrap());
    if let RunPart::Workout { ref mut nodes, .. } = w {
        *nodes = parts;
    }
    Ok((rem_input, w))
}

fn parse_parts(input: &str) -> IResult<&str, Vec<RunPart>> {
    // part, { "+", part }
    separated_list1(tag("+"), parse_part)(input)
}

fn parse_part(input: &str) -> IResult<&str, RunPart> {
    // <workout> | <step>
    // TODO: try alt
    let res_w = parse_workout(input);
    if let Ok((rem_input, workout)) = res_w {
        return Ok((rem_input, workout));
    } else {
        parse_step(input)
    }
}

fn parse_step(input: &str) -> IResult<&str, wtree::RunPart> {
    // <time step> | <distance step>
    alt((parse_time_step, parse_distance_step))(input)
}

fn parse_distance_step(input: &str) -> IResult<&str, wtree::RunPart> {
    // <distance> <effort>
    let (rem_input, (distance, effort)) = tuple((parse_distance, parse_effort))(input)?;
    info!("New distance step from: {}", input);
    if distance < 100.0 {
        Ok((
            rem_input,
            wtree::RunPart::part_from_distance(distance * 1000.0, pace2speed(get_pace(effort))),
        ))
    } else {
        Ok((
            rem_input,
            wtree::RunPart::part_from_distance(distance, pace2speed(get_pace(effort))),
        ))
    }
}

fn parse_time_step(input: &str) -> IResult<&str, wtree::RunPart> {
    // <time [min]> <effort>
    let (rem_input, (time, effort)) = tuple((parse_time, parse_effort))(input)?;
    info!("New time step from: {}", input);
    Ok((
        rem_input,
        wtree::RunPart::part_from_time(time, pace2speed(get_pace(effort))),
    ))
}

fn parse_distance(input: &str) -> IResult<&str, f32> {
    let (input, distance) = take_while(is_float_digit)(input)?;
    Ok((input, distance.parse::<f32>().unwrap()))
}

fn parse_time(input: &str) -> IResult<&str, f32> {
    let (input, (time, unit)) =
        tuple((take_while(is_float_digit), alt((tag("min"), tag("s")))))(input)?;
    Ok((
        input,
        time.parse::<f32>().unwrap() * {
            if unit.contains("min") {
                60.0
            } else {
                1.0
            }
        },
    ))
}

fn parse_effort(input: &str) -> IResult<&str, &str> {
    // the alt combinator requires its alternaitve parsers during compile time. Since
    // we like to enable the definition during runtime in the config, we have to reimplement it
    for i in get_intensities() {
        match tag(i.as_str())(input) {
            Ok((rem_input, effort)) => return Ok((rem_input, effort)),
            Err(Err::Error((_, ErrorKind::Tag))) => (),
            _ => (),
        }
    }
    // no match found
    Err(Err::Error(Error::new(
        "Intensity not found",
        ErrorKind::Tag,
    )))
}

fn is_float_digit(c: char) -> bool {
    c.is_ascii_digit() || c == '.'
}

// --- tests -----------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn single_step_1() {
        let (_, w) = parse_step("3E").unwrap();
        assert_abs_diff_eq!(w.calc_distance(), 3000_f32);
        assert_abs_diff_eq!(w.calc_time(), (3 * 6 * 60) as f32);
    }

    #[test]
    fn single_step_2() {
        let (_, s) = parse_step("360sE").unwrap();
        assert_abs_diff_eq!(s.calc_distance(), 1000_f32, epsilon = 0.1);
        assert_abs_diff_eq!(s.calc_time(), 360_f32);
    }

    #[test]
    fn single_step_workout() {
        let (_, w) = parse_workout(&normalize_input("3jog")).unwrap();

        if let RunPart::Workout { ref nodes, .. } = w {
            assert_eq!(nodes.len(), 1);
            assert_abs_diff_eq!(w.calc_distance(), 3000_f32, epsilon = 0.1);
            assert_abs_diff_eq!(w.calc_time(), (3 * 8 * 60) as f32, epsilon = 0.1);
        }
    }
    #[test]
    fn multi_step_workout() {
        let (_, w) = parse_workout(&normalize_input("3 M + 3 T")).unwrap();
        if let RunPart::Workout { ref nodes, .. } = w {
            assert_eq!(nodes.len(), 2);
            assert_abs_diff_eq!(w.calc_distance(), 6000_f32, epsilon = 0.1);
            assert_abs_diff_eq!(
                w.calc_time(),
                (3 * (5 * 60) + 3 * (4 * 60 + 30)) as f32,
                epsilon = 0.1
            );
        }
    }

    #[test]
    fn repeats() {
        let (_, w) = parse_workout(&normalize_input("2min I + 3*(1min H + 5min jg)")).unwrap();
        if let RunPart::Workout { ref nodes, .. } = w {
            assert_eq!(nodes.len(), 2);
            assert_abs_diff_eq!(
                w.calc_time(),
                ((2 + 3 * (1 + 5)) * 60) as f32,
                epsilon = 0.1
            );
        }
    }

    #[test]
    fn repeats_2() {
        let (_, w) = parse_workout(&normalize_input(
            "10 min E + 5 * (3 min I + 2 min jg) + 6 * (1 min R + 2 min jg)",
        ))
        .unwrap();

        if let RunPart::Workout { ref nodes, .. } = w {
            assert_eq!(nodes.len(), 3);
            assert_abs_diff_eq!(
                w.calc_time(),
                ((10 + 5 * (3 + 2) + 6 * (1 + 2)) * 60) as f32,
                epsilon = 0.1
            );
        }
    }

    #[test]
    fn summaries() {
        assert_eq!(
            summarize("3.2E + 2 * (1.6T + 1 min rest) + 30min E + 2 * (1.6T + 1 min rest) + 3.2E"),
            "18.1 km, 1:41 h, 5:36 min/km".to_string()
        );
        assert_eq!(
            summarize("2E + 2 * ( 5 * (4 min I + 90s jg)) + 2 E"),
            "15.9 km, 1:19 h, 4:58 min/km".to_string()
        );
    }
}
