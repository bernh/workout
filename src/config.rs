
#[cfg(not(test))]
pub fn get_pace(effort: &str) -> String {
    match effort {
        "E"    => "5:30",
        "M"    => "4:57",
        "T"    => "4:38",
        "I"    => "4:16",
        "H"    => "4:16",
        "jg"   => "6:30",
        "jog"  => "6:30",
        "rst"  => "15:00",
        "rest" => "15:00",
        _      => "15:00",  // TODO panic

    }.to_string()
}


#[cfg(test)]
pub fn get_pace(effort: &str) -> String {
    match effort {
        "E"    => "6:00",
        "M"    => "5:00",
        "T"    => "4:30",
        "I"    => "4:00",
        "H"    => "4:00",
        "jg"   => "8:00",
        "jog"  => "8:00",
        "rst"  => "15:00",
        "rest" => "15:00",
        _      => "15:00",  // TODO panic
    }.to_string()
}
