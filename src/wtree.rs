#[derive(Debug, Clone)]
pub enum RunType {
    Distance,
    Time,
}

pub trait DistanceAndTime {
    fn time(&self) -> f32;
    fn distance(&self) -> f32;
}

#[derive(Debug, Clone)]
pub struct Run {
    pub rtype: RunType, // not really needed but nice nevertheless for better FIT files
    pub speed: f32, // m/s
    pub time: f32, // s
    pub distance: f32, // m
}

pub struct Workout {
    pub reps: i32,
    pub nodes: Vec<Box<DistanceAndTime>>,
}

impl Run {
    pub fn from_distance(distance: f32, speed: f32) -> Run {
        let time = distance / speed;
        Run {
            rtype: RunType::Distance,
            speed: speed,
            time: time,
            distance: distance,
        }
    }
    pub fn from_time(time: f32, speed: f32) -> Run {
        let distance = time * speed;
        Run {
            rtype: RunType::Time,
            speed: speed,
            time: time,
            distance: distance,
        }
    }
}

impl DistanceAndTime for Run {
    fn time(&self) -> f32 {
        self.time
    }
    fn distance(&self) -> f32 {
        self.distance
    }
}

impl Workout {
    pub fn new(reps: i32) -> Workout {
        Workout {
            reps: reps,
            nodes: Vec::new(),
        }
    }

    pub fn add<T: DistanceAndTime + 'static>(&mut self, node: T) {
        self.nodes.push(Box::new(node));
    }

    // pub fn pace<T: DistanceAndTime>(&self) -> String {
    //     speed2pace(self.distance() / self.speed())
    // }
}

impl DistanceAndTime for Workout {
    fn time(&self) -> f32 {
        self.reps as f32 * self.nodes.iter().fold(0.0, |acc, ref x| acc + x.time())
    }
    fn distance(&self) -> f32 {
        self.reps as f32 * self.nodes.iter().fold(0.0, |acc, ref x| acc + x.distance())
    }
}

pub fn pace2speed(pace: String) -> f32 {
    // pace is min:sec per kilometer, speed is m/s
    let values: Vec<_> = pace.split(":").collect();
    let seconds = values[0].parse::<i32>().unwrap() * 60 + values[1].parse::<i32>().unwrap();
    1000.0 / seconds as f32
}

pub fn speed2pace(speed: f32) -> String {
    let seconds = (1000.0 / speed) as i32;
    let mins = seconds / 60;
    let remaining = seconds % 60 as i32;
    format!("{}:{:02}", mins, remaining)
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_delta {
        ($x:expr, $y:expr, $d:expr) => {
            if !($x - $y < $d || $y - $x < $d) { panic!(); }
        }
    }

    #[test]
    fn pace_speed_convert() {
        assert_delta!(pace2speed("6:00".to_string()), 10.0 / 3.6, 0.1);
        assert_eq!(speed2pace(2.778), "5:59");
    }

    #[test]
    fn totals() {
        let mut t = Workout::new(2);
        t.add(Run::from_distance(1000.0, pace2speed("5:00".to_string())));
        t.add(Run::from_time(240.0, pace2speed("4:00".to_string())));
        assert_delta!(t.time(), 1080.0, 0.1);
        assert_delta!(t.distance(), 4000.0, 0.1);
        // TODO assert_eq!(t.pace(), "4:30".to_string());
    }

}
