use std::collections::HashMap;

// extern crates
use once_cell::sync::OnceCell;

static CONFIG: OnceCell<HashMap<String, String>> = OnceCell::new();

pub fn init(paces: HashMap<String, String>) {
    CONFIG.set(paces).unwrap();
}

#[cfg(not(test))]
pub fn get_pace(effort: &str) -> &str {
    CONFIG.get().unwrap()[effort].as_str()
}

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
mod tests {
    use super::*;

    #[test]
    fn test_get_pace() {
        assert_eq!("4:00", get_pace("H"));
    }
}
