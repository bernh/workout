use crate::wtree;
use crate::wtree::DistanceAndTime;

use log::*;


pub fn log_parse(input: String) {
    let w = parse_workout(&input).unwrap();
    info!("{}", input);
    info!(
        "({:.*} km, {}:{:02} h)",
        1,
        w.distance() as f32 / 1000.0,
        w.time() as i32 / 3600,
        w.time() as i32 % 3600 / 60
    );
}



pub fn parse_step(step: &str) -> Option<wtree::Run> {
    debug!("parsing step: {}", step);
    todo!();
}

pub fn parse_workout(workout: &str) -> Option<wtree::Workout> {
    debug!("parsing workout: {}", workout);
    todo!();
}


#[cfg(test)]
mod tests {
    use super::*;
    use wtree::DistanceAndTime;

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) { panic!(); }
        }
    }

    #[test]
    fn single_step() {
        let r = parse_step("3E");
        let w = r.unwrap();
        assert_eq!(w.distance(), 3000.0);
        assert_delta!(w.time(), (3 * 6 * 60) as f32, 0.1);
    }

    #[test]
    fn single_step_workout() {
        let r = parse_workout("3jog");
        let w = r.unwrap();
        assert_eq!(w.nodes.len(), 1);
        assert_delta!(w.distance(), 3000 as f32, 0.1);
        assert_delta!(w.time(), (3 * 8 * 60) as f32, 0.1);
    }

    #[test]
    fn multi_step_workout() {
        let r = parse_workout("3M+3T");
        let w = r.unwrap();
        assert_eq!(w.nodes.len(), 2);
        assert_delta!(w.distance(), 6000 as f32, 0.1);
        assert_delta!(w.time(), (3 * (5 * 60) + 3 * (4 * 60 + 30)) as f32, 0.1);
    }

    #[test]
    fn repeats() {
        let r = parse_workout("2min I + 3*(1min H + 5min jg)");
        let w = r.unwrap();
        assert_eq!(w.nodes.len(), 2);
        assert_delta!(w.time(), ((2 + 3 * (1 + 5)) * 60) as f32, 0.1);
    }
}
