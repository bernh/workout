
#[cfg(not(test))]
pub fn get_pace(effort: &str) -> String {
    match effort {
        "E"    => "5:30",
        "M"    => "5:00",
        "T"    => "4:30",
        "I"    => "4:00",
        "H"    => "4:00",
        "jg"   => "8:00",
        "jog"  => "8:00",
        "rst"  => "10:00",
        "rest" => "10:00",
        _      => "10:00",  // TODO panic

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
        "rst"  => "10:00",
        "rest" => "10:00",
        _      => "10:00",  // TODO panic
    }.to_string()
}
