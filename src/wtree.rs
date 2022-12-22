use std::fmt;

#[derive(Debug, Clone)]
pub enum RunType {
    Distance,
    Time,
}

pub trait DistanceAndTime: fmt::Display {
    fn time(&self) -> f32;
    fn distance(&self) -> f32;
}

#[derive(Debug, Clone)]
pub struct Step {
    pub rtype: RunType, // based on distance or time
    pub speed: f32,     // m/s
    pub time: f32,      // s
    pub distance: f32,  // m
}

pub struct Workout {
    pub reps: i32,
    pub nodes: Vec<Box<dyn DistanceAndTime>>, // vector of trait objects
}

impl Step {
    pub fn from_distance(distance: f32, speed: f32) -> Step {
        let time = distance / speed;
        Step {
            rtype: RunType::Distance,
            speed,
            time,
            distance,
        }
    }
    pub fn from_time(time: f32, speed: f32) -> Step {
        let distance = time * speed;
        Step {
            rtype: RunType::Time,
            speed,
            time,
            distance,
        }
    }
}

impl DistanceAndTime for Step {
    fn time(&self) -> f32 {
        self.time
    }
    fn distance(&self) -> f32 {
        self.distance
    }
}

impl fmt::Display for Step {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.rtype {
            RunType::Distance => write!(
                f,
                "{:.*} km @ {} min/km pace",
                1,
                self.distance() / 1000.0,
                speed2pace(self.speed)
            ),
            RunType::Time => write!(
                f,
                "{}:{:02} min @ {} min/km pace",
                self.time() as i32 / 60,
                self.time() as i32 % 60,
                speed2pace(self.speed)
            ),
        }
    }
}

impl Workout {
    pub fn new(reps: i32) -> Workout {
        Workout {
            reps,
            nodes: Vec::new(),
        }
    }
}

impl DistanceAndTime for Workout {
    fn time(&self) -> f32 {
        self.reps as f32 * self.nodes.iter().fold(0.0, |acc, x| acc + x.time())
    }
    fn distance(&self) -> f32 {
        self.reps as f32 * self.nodes.iter().fold(0.0, |acc, x| acc + x.distance())
    }
}

impl fmt::Display for Workout {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        writeln!(f, "\n{} * (", self.reps)?;
        for n in self.nodes.iter() {
            writeln!(f, "  {}", n)?;
        }
        writeln!(f, ")")
    }
}

pub fn pace2speed(pace: &str) -> f32 {
    // pace is min:sec per kilometer, speed is m/s
    let values: Vec<_> = pace.split(':').collect();
    let seconds = values[0].parse::<i32>().unwrap() * 60 + values[1].parse::<i32>().unwrap();
    1000.0 / seconds as f32
}

pub fn speed2pace(speed: f32) -> String {
    let seconds = (1000.0 / speed) as i32;
    let mins = seconds / 60;
    let remaining = seconds % 60_i32;
    format!("{}:{:02}", mins, remaining)
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_abs_diff_eq;

    #[test]
    fn pace_speed_convert() {
        assert_abs_diff_eq!(pace2speed("6:00"), 10.0 / 3.6, epsilon = 0.1);
        assert_eq!(speed2pace(2.778), "5:59");
    }

    #[test]
    fn totals() {
        let mut t = Workout::new(2);
        t.nodes
            .push(Box::new(Step::from_distance(1000.0, pace2speed("5:00"))));
        t.nodes
            .push(Box::new(Step::from_time(240.0, pace2speed("4:00"))));
        assert_abs_diff_eq!(t.time(), 1080.0);
        assert_abs_diff_eq!(t.distance(), 4000.0);
        // TODO assert_eq!(t.pace(), "4:30");
    }
}
