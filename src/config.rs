use std::env;
use std::fs::File;
use std::io::Read;

// extern crates
use lazy_static::*;
use toml;

lazy_static! {
    static ref CONFIG: toml::Value = {
        let c = env::var("WORKOUT_CONFIG_FILE").expect("Environment variable not set");
        let mut f = File::open(c).expect("Couldn't open config file");
        let mut s = String::new();
        f.read_to_string(&mut s).unwrap();
        toml::from_str(&s).unwrap()
    };
}

pub fn init(c: &str) {
    // Is there a better way to pass information to the lazy_static than via env variables?
    // Anyway, it's probably good enough for this use case. init does not need to be thread save.
    env::set_var("WORKOUT_CONFIG_FILE", c);
}

#[cfg(not(test))]
pub fn get_pace(effort: &str) -> String {
    (*CONFIG["paces"][effort].as_str().unwrap()).to_string()
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
