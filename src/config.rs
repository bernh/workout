use std::env;
use std::fs::File;
use std::io::Read;

// extern crates
use once_cell::sync::OnceCell;

static CONFIG: OnceCell<toml::Value> = OnceCell::new();

pub fn init(c: &str) {
    let mut f = File::open(c).expect("Couldn't open config file");
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    CONFIG.set(toml::from_str(&s).unwrap()).unwrap();
}

#[cfg(not(test))]
pub fn get_pace(effort: &str) -> String {
    (CONFIG.get().unwrap()["paces"][effort].as_str().unwrap()).to_string()
}

#[cfg(test)]
pub fn get_pace(effort: &str) -> String {
    let inline_config = r##"
        [paces]
        E = "6:00"
        M = "5:00"
        T = "4:30"
        I = "4:00"
        H = "4:00"
        R = "3:30"
        jg = "8:00"
        jog = "8:00"
        rst = "15:00"
        rest = "15:00"
        "##;
    let config: toml::Value = toml::from_str(inline_config).unwrap();
    let pace = config["paces"][effort].as_str().unwrap();
    pace.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pace() {
        assert_eq!("4:00", get_pace("H"));
    }
}
