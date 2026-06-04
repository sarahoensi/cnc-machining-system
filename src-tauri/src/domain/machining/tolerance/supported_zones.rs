pub const REQUIRED_HOLE_ZONES: &[&str] = &[
    "D", "E", "F", "G", "H", "J", "JS", "K", "M", "N", "P", "R", "S", "T", "U", "V", "X", "Y", "Z",
    "ZA", "ZB", "ZC",
];

pub const REQUIRED_SHAFT_ZONES: &[&str] = &["f", "g", "h", "js", "k", "m", "n", "p", "r"];

pub fn is_allowed_zone(feature: &str, zone: &str) -> bool {
    match feature {
        "hole" => REQUIRED_HOLE_ZONES.contains(&zone),
        "shaft" => REQUIRED_SHAFT_ZONES.contains(&zone),
        _ => false,
    }
}
