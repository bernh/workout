pub fn pace2speed(pace: &str) -> Option<f32> {
    // pace is min:sec per kilometer, speed is m/s
    let values: Vec<_> = pace.split(':').collect();
    if values.len() != 2 {
        return None;
    }
    let minutes = values[0].parse::<i32>();
    let seconds = values[1].parse::<i32>();
    match (minutes, seconds) {
        (Ok(min), Ok(sec)) => Some(1000.0 / (min * 60 + sec) as f32),
        _ => None,
    }
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
        assert_abs_diff_eq!(pace2speed("6:00").unwrap(), 10.0 / 3.6, epsilon = 0.1);
        assert_eq!(speed2pace(2.778), "5:59");
    }

    #[test]
    fn invalid_pace() {
        assert_eq!(pace2speed("23:a"), None);
        assert_eq!(pace2speed("b:02a"), None);
        assert_eq!(pace2speed("b"), None);
    }
}
