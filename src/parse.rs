use crate::wtree;

pub fn parse_Step(step: &str) -> Option<wtree::Run> {
    todo!();
}

pub fn parse_Workout(workout: &str) -> Option<wtree::Workout> {
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
        let r = parse_Step("3E");
        let w = r.unwrap();
        assert_eq!(w.distance(), 3000.0);
        assert_delta!(w.time(), (3 * 6 * 60) as f32, 0.1);
    }

    #[test]
    fn single_step_workout() {
        let r = parse_Workout("3jog");
        let w = r.unwrap();
        assert_eq!(w.nodes.len(), 1);
        assert_delta!(w.distance(), 3000 as f32, 0.1);
        assert_delta!(w.time(), (3 * 8 * 60) as f32, 0.1);
    }

    #[test]
    fn multi_step_workout() {
        let r = parse_Workout("3M+3T");
        let w = r.unwrap();
        assert_eq!(w.nodes.len(), 2);
        assert_delta!(w.distance(), 6000 as f32, 0.1);
        assert_delta!(w.time(), (3 * (5 * 60) + 3 * (4 * 60 + 30)) as f32, 0.1);
    }

    #[test]
    fn repeats() {
        let r = parse_Workout("2min I + 3*(1min H + 5min jg)");
        let w = r.unwrap();
        assert_eq!(w.nodes.len(), 2);
        assert_delta!(w.time(), ((2 + 3 * (1 + 5)) * 60) as f32, 0.1);
    }
}
