use std::fmt;

use crate::utils::*;

#[derive(Debug, Clone)]
pub enum RunType {
    Distance,
    Time,
}

//  https://users.rust-lang.org/t/could-enums-be-considered-an-anti-pattern/10068/3
// "Enums are for closed sets where you know every variant while trait objects are for
// open sets."
//
// Original implementation used trait objects for Step and Workout, changed to enum
// because we are dealing with a "closed set". Not sure if the code is really cleaner now.
#[derive(Debug, Clone)]
pub enum RunPart {
    Step {
        rtype: RunType, // based on distance or time
        speed: f32,     // m/s
        time: f32,      // s
        distance: f32,  // m
    },
    Workout {
        reps: i32,
        nodes: Vec<RunPart>,
    },
}
use RunPart::{Step, Workout};

impl RunPart {
    pub fn part_from_distance(distance: f32, speed: f32) -> RunPart {
        let time = distance / speed;
        Step {
            rtype: RunType::Distance,
            speed,
            time,
            distance,
        }
    }

    pub fn part_from_time(time: f32, speed: f32) -> RunPart {
        let distance = time * speed;
        Step {
            rtype: RunType::Time,
            speed,
            time,
            distance,
        }
    }

    pub fn new_workout(reps: i32) -> RunPart {
        Workout {
            reps,
            nodes: Vec::new(),
        }
    }

    pub fn calc_time(&self) -> f32 {
        match self {
            Step { time, .. } => *time,
            Workout { reps, nodes } => {
                *reps as f32 * nodes.iter().fold(0.0, |acc, x| acc + x.calc_time())
            }
        }
    }

    pub fn calc_distance(&self) -> f32 {
        match self {
            Step { distance, .. } => *distance,
            Workout { reps, nodes } => {
                *reps as f32 * nodes.iter().fold(0.0, |acc, x| acc + x.calc_distance())
            }
        }
    }
}

impl fmt::Display for RunPart {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Step {
                rtype,
                distance,
                speed,
                time,
            } => match rtype {
                RunType::Distance => write!(
                    f,
                    "{:.*} km @ {} min/km pace",
                    1,
                    distance / 1000.0,
                    speed2pace(*speed)
                ),
                RunType::Time => write!(
                    f,
                    "{}:{:02} min @ {} min/km pace",
                    *time as i32 / 60,
                    *time as i32 % 60,
                    speed2pace(*speed)
                ),
            },
            Workout { reps, nodes } => {
                writeln!(f, "\n{} * (", reps)?;
                for n in nodes.iter() {
                    writeln!(f, "  {}", n)?;
                }
                writeln!(f, ")")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn totals() {
        let mut t = RunPart::new_workout(2);
        if let RunPart::Workout { ref mut nodes, .. } = t {
            nodes.push(RunPart::part_from_distance(1000.0, pace2speed("5:00")));
            nodes.push(RunPart::part_from_time(240.0, pace2speed("4:00")));
            assert_abs_diff_eq!(t.calc_time(), 1080.0);
            assert_abs_diff_eq!(t.calc_distance(), 4000.0);
        }
        // TODO assert_eq!(t.pace(), "4:30");
    }
}
