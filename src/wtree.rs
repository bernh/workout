#[derive(Debug)]
enum RunType {
    Distance,
    Time,
}

pub trait Distance_and_Time {
    fn time(&self) -> f32;
    fn distance(&self) -> f32;
}

#[derive(Debug)]
pub struct Run {
    rtype: RunType,  // not really needed but nice nevertheless for better FIT files
    speed: f32, // m/s
    time: f32, // s
    distance: f32, // m
}

pub struct Workout {
    reps: usize,
    nodes: Vec<Box<Distance_and_Time>>,
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

impl Distance_and_Time for Run {
    fn time(&self) -> f32 {
        self.time
    }
    fn distance(&self) -> f32 {
        self.distance
    }
}

impl Workout {
    pub fn new(reps: usize) -> Workout {
        Workout {
            reps: reps,
            nodes: Vec::new(),
        }
    }

    pub fn add<T: Distance_and_Time + 'static>(&mut self, node: T) {
        self.nodes.push(Box::new(node));
    }

    pub fn pace(&self) -> String {
        // TODO average pace
        "3:00".to_string()
    }
}

impl Distance_and_Time for Workout {
    fn time(&self) -> f32 {
        self.reps as f32 * self.nodes.iter().fold(0.0, |acc, ref x| acc + x.time())
    }
    fn distance(&self) -> f32 {
        self.reps as f32 * self.nodes.iter().fold(0.0, |acc, ref x| acc + x.distance())
    }
}


pub fn debug_test() {
    let mut t2 = Workout::new(2);
    t2.add(Run::from_distance(1000.0, pace2speed("5:00".to_string())));
    t2.add(Run::from_time(240.0, pace2speed("4:00".to_string())));
    println!("    total time: {}", t2.time());
    println!("    total distance: {}", t2.distance());
}

fn pace2speed(pace: String) -> f32 {
    // pace is min:sec per kilometer, speed is m/s
    let values: Vec<_> = pace.split(":").collect();
    let seconds = values[0].parse::<i32>().unwrap() * 60 + values[1].parse::<i32>().unwrap();
    1000.0 / seconds as f32
}

fn speed2pace(speed: f32) -> String {
    let seconds = (1000.0 / speed) as i32;
    let mins = seconds / 60;
    let remaining = seconds % 60 as i32;
    format!("{}:{:02}", mins, remaining)
}


#[cfg(test)]

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

// #[test]
// fn totals() {
//     let mut t2 = Workout::new(2);
//     t2.add(Node::Step(Run::from_distance(1000.0, pace2speed("5:00".to_string()))));
//     t2.add(Node::Step(Run::from_time(240 as f32, pace2speed("4:00".to_string()))));
//     assert_delta!(t2.time(), 1080.0, 0.1);
//     assert_delta!(t2.distance(), 4000.0, 0.1);
// }

#[test]
fn construct() {
    // 1 E + 2 * (1 T + 1 min rest) + 3 * (3 min H + 2 min jg)
    // + 4 * (200 R + 200 jg) + 1 E
    assert_eq!(1, 1);
}
