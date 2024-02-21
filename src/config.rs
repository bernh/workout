use std::collections::HashMap;
use std::sync::Mutex;

use once_cell::sync::Lazy;

static CONFIG: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(|| Mutex::new(HashMap::new()));

pub fn init(paces: HashMap<String, String>) {
    let mut config = CONFIG.lock().unwrap();
    *config = paces;
}

#[cfg(not(test))]
pub fn get_pace(effort: &str) -> String {
    CONFIG.lock().unwrap()[effort].clone()
}

#[cfg(not(test))]
pub fn get_intensities() -> Vec<String> {
    CONFIG.lock().unwrap().keys().cloned().collect()
}

// unit tests use a hard-coded config
#[cfg(test)]
pub fn get_pace(effort: &str) -> &str {
    let inline_config: HashMap<&str, &str> = HashMap::from([
        ("E", "6:00"),
        ("M", "5:00"),
        ("T", "4:30"),
        ("I", "4:00"),
        ("H", "4:00"),
        ("R", "3:30"),
        ("jg", "8:00"),
        ("jog", "8:00"),
        ("rst", "15:00"),
        ("rest", "15:00"),
    ]);
    inline_config[effort]
}

#[cfg(test)]
pub fn get_intensities() -> Vec<String> {
    let intensities = ["E", "M", "T", "I", "H", "R", "jg", "jog", "rst", "rest"];
    intensities.iter().map(|s| s.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pace() {
        assert_eq!("4:00", get_pace("H"));
    }
}
