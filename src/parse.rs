use crate::config::{get_intensities, get_pace};
use crate::utils::pace2speed;
use crate::wtree::{self, RunPart};

use log::info;
use winnow::ascii::digit1;
use winnow::combinator::separated;
use winnow::token::take_while;
use winnow::{
    combinator::{alt, fail},
    PResult, Parser,
};

pub fn summarize(input: &str) -> Option<String> {
    match parse_workout(&mut normalize_input(input).as_str()) {
        Ok(w) => {
            info!("{}", w);
            Some(format!(
                "{:.*} km, {}:{:02} h, {}:{:02} min/km",
                1,
                w.calc_distance() / 1000.0,
                w.calc_time() as i32 / 3600,
                w.calc_time() as i32 % 3600 / 60,
                (w.calc_time() / (w.calc_distance() / 1000.0)) as i32 / 60,
                (w.calc_time() / (w.calc_distance() / 1000.0)) as i32 % 60,
            ))
        }
        Err(_) => None,
    }
}

fn normalize_input(input: &str) -> String {
    let norm: String = input.split_whitespace().collect();
    // top level may be list of parts or a repetion. Normalize to always start with a
    // repetition
    format!("1*({})", norm)
}

// --- winnow parser combinator functions ---

pub fn parse_workout(input: &mut &str) -> PResult<wtree::RunPart> {
    // <rep> "*" "("<parts>")"
    let (rep, _, _, parts, _) = (digit1, "*", "(", parse_parts, ")").parse_next(input)?;
    info!("New Workout from: {}", input);
    let mut w = RunPart::new_workout(rep.parse::<i32>().unwrap());
    if let RunPart::Workout { ref mut nodes, .. } = w {
        *nodes = parts;
    }
    Ok(w)
}

fn parse_parts(input: &mut &str) -> PResult<Vec<RunPart>> {
    // part, { "+", part }
    separated(1.., parse_part, "+").parse_next(input)
}

fn parse_part(input: &mut &str) -> PResult<RunPart> {
    // <workout> | <step>
    alt((parse_workout, parse_step)).parse_next(input)
}

fn parse_step(input: &mut &str) -> PResult<wtree::RunPart> {
    // <time step> | <distance step>
    alt((parse_time_step, parse_distance_step)).parse_next(input)
}

fn parse_distance_step(input: &mut &str) -> PResult<wtree::RunPart> {
    // <distance> <effort>
    let (distance, effort) = (parse_distance, parse_effort).parse_next(input)?;
    info!("New distance step from: {}", input);
    if distance < 100.0 {
        // distances below 100 meters (or above 100 km) will be misinterpreted
        Ok(wtree::RunPart::part_from_distance(
            distance * 1000.0,
            pace2speed(&get_pace(effort)).unwrap(),
        ))
    } else {
        Ok(wtree::RunPart::part_from_distance(
            distance,
            pace2speed(&get_pace(effort)).unwrap(),
        ))
    }
}

fn parse_time_step(input: &mut &str) -> PResult<wtree::RunPart> {
    // <time [min|s]> <effort>
    let (time, effort) = (parse_time, parse_effort).parse_next(input)?;
    info!("New time step from: {}", input);
    Ok(wtree::RunPart::part_from_time(
        time,
        pace2speed(&get_pace(effort)).unwrap(),
    ))
}

fn parse_distance(input: &mut &str) -> PResult<f32> {
    // <distance>
    let distance: &str = take_while(1.., is_float_digit).parse_next(input)?;
    Ok(distance.parse::<f32>().unwrap())
}

fn parse_time(input: &mut &str) -> PResult<f32> {
    // <time [min|s]>
    let time: &str = take_while(1.., is_float_digit).parse_next(input)?;
    let unit: &str = alt(("min", "s", fail)).parse_next(input)?;
    Ok(time.parse::<f32>().unwrap()
        * match unit {
            "min" => 60.0,
            "s" => 1.0,
            _ => panic!("unknown time unit"), // should never happen
        })
}

fn is_float_digit(c: char) -> bool {
    c.is_ascii_digit() || c == '.'
}

fn parse_effort<'s>(input: &mut &'s str) -> PResult<&'s str> {
    // the alt combinator requires its alternatives in a tuple at compile time.
    // Since our intensities are only known during runtime we can't use it here.

    for i in get_intensities() {
        let intensity: PResult<&str> = i.as_str().parse_next(input);
        match intensity {
            Ok(x) => return Ok(x),
            _ => (),
        }
    }
    fail(input)
}

// --- tests -----------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;
    #[test]
    fn test_parse_distance() {
        // Ok
        let mut input = "1.6";
        assert_abs_diff_eq!(parse_distance(&mut input).unwrap(), 1.6f32, epsilon = 0.01);
        // Err
        let mut input = "E3";
        assert!(parse_distance(&mut input).is_err());
    }

    #[test]
    fn test_parse_time() {
        // Ok
        let mut input = "20min";
        assert_abs_diff_eq!(parse_time(&mut input).unwrap(), 20.0 * 60.0, epsilon = 0.01);
        let mut input = "1.5min";
        assert_abs_diff_eq!(parse_time(&mut input).unwrap(), 90.0, epsilon = 0.01);
        let mut input = "60s";
        assert_abs_diff_eq!(parse_time(&mut input).unwrap(), 60.0, epsilon = 0.01);
        // Err
        let mut input = "20";
        assert!(parse_time(&mut input).is_err());
        let mut input = "20h";
        assert!(parse_time(&mut input).is_err());
    }

    #[test]
    fn test_parse_effort() {
        // Ok
        let mut input = "E+20minT";
        assert_eq!(parse_effort(&mut input).unwrap(), "E");
        // Err
        let mut input = "foo+20minT";
        assert!(parse_effort(&mut input).is_err());
    }

    #[test]
    fn test_parse_time_step() {
        // Ok
        let mut input = "360sE";
        let s = parse_time_step(&mut input).unwrap();
        assert_abs_diff_eq!(s.calc_distance(), 1000.0, epsilon = 0.01);
        assert_abs_diff_eq!(s.calc_time(), 360.0, epsilon = 0.01);
        // Err
        let mut input = "20hE";
        assert!(parse_time_step(&mut input).is_err());
    }

    #[test]
    fn test_parse_distance_step() {
        let mut input = "3E";
        let s = parse_distance_step(&mut input).unwrap();
        assert_abs_diff_eq!(s.calc_distance(), 3000.0, epsilon = 0.01);
        assert_abs_diff_eq!(s.calc_time(), 3.0 * 6.0 * 60.0, epsilon = 0.01);

        let mut input = "400R";
        let s = parse_distance_step(&mut input).unwrap();
        assert_abs_diff_eq!(s.calc_distance(), 400.0, epsilon = 0.01);
    }

    #[test]
    fn single_step_workout() {
        let w = parse_workout(&mut normalize_input("3jog").as_str()).unwrap();

        if let RunPart::Workout { ref nodes, .. } = w {
            assert_eq!(nodes.len(), 1);
            assert_abs_diff_eq!(w.calc_distance(), 3000_f32, epsilon = 0.1);
            assert_abs_diff_eq!(w.calc_time(), (3 * 8 * 60) as f32, epsilon = 0.1);
        }
    }

    #[test]
    fn multi_step_workout() {
        let w = parse_workout(&mut normalize_input("3 M + 3 T").as_str()).unwrap();
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
        let w =
            parse_workout(&mut normalize_input("2min I + 3*(1min H + 5min jg)").as_str()).unwrap();
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
        let w = parse_workout(
            &mut normalize_input("10 min E + 5 * (3 min I + 2 min jg) + 6 * (1 min R + 2 min jg)")
                .as_str(),
        )
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
            summarize("3.2E + 2 * (1.6T + 1 min rest) + 30min E + 2 * (1.6T + 1 min rest) + 3.2E")
                .unwrap(),
            "18.1 km, 1:41 h, 5:36 min/km".to_string()
        );
        assert_eq!(
            summarize("2E + 2 * ( 5 * (4 min I + 90s jg)) + 2 E").unwrap(),
            "15.9 km, 1:19 h, 4:58 min/km".to_string()
        );
    }

    #[test]
    fn summary_invalid_inputs() {
        // basically make sure that we are not crashing with invalid input
        assert_eq!(summarize("bla"), None);
    }
}
