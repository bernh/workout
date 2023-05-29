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
}
