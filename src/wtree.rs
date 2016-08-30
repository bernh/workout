#[derive(Debug)]
enum RunType {
    Distance,
    Time,
}

#[derive(Debug)]
pub struct Run {
    rtype: RunType,
    speed: f32, // m/s
    time: f32, // s
    distance: f32, // m
}

impl Run {
    pub fn distance(distance: f32, speed: f32) -> Run {
        let time = distance / speed;
        Run {
            rtype: RunType::Distance,
            speed: speed,
            time: time,
            distance: distance,
        }
    }
    pub fn time(time: f32, speed: f32) -> Run {
        let distance = time * speed;
        Run {
            rtype: RunType::Time,
            speed: speed,
            time: time,
            distance: distance,
        }
    }
}


#[derive(Debug)]
pub struct Workout {
    reps: usize,
    nodes: Vec<Node>,
}

#[derive(Debug)]
enum Node {
    Nested(Workout),
    Step(Run),
}

pub fn debug_test() {
    let t = Workout{reps: 1, 
                    nodes: vec![Node::Step(Run::distance(1.0, 2.0)),
                                Node::Step(Run::time(34.0, 2.4)),
                                Node::Nested(Workout{reps: 2,
                                                     nodes: vec![Node::Step(Run::distance(1.0, 2.0))]}),
                    ]};
    println!("Workout: {:?}", t);
}

#[cfg(test)]
#[test]
fn construct() {
    // 1 E + 2 * (1 T + 1 min rest) + 3 * (3 min H + 2 min jg)
    // + 4 * (200 R + 200 jg) + 1 E
    assert_eq!(1, 1);
}
