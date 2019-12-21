#[cfg(not(test))]
pub fn get_pace(effort: &str) -> String {
    match effort {
        "E" => "5:30",
        "M" => "4:55", // VDOT: 45.3
        "T" => "4:33", // VDOT: 46
        "I" => "4:12", // VDOT: 46
        "H" => "4:12", // VDOT: 46
        "R" => "3:50", // VDOT: 46
        "jg" => "6:30",
        "jog" => "6:30",
        "rst" => "15:00",
        "rest" => "15:00",
        _ => "15:00", // TODO panic
    }
    .to_string()
}

#[cfg(test)]
pub fn get_pace(effort: &str) -> String {
    match effort {
        "E" => "6:00",
        "M" => "5:00",
        "T" => "4:30",
        "I" => "4:00",
        "H" => "4:00",
        "jg" => "8:00",
        "jog" => "8:00",
        "rst" => "15:00",
        "rest" => "15:00",
        _ => "15:00", // TODO panic
    }
    .to_string()
}
