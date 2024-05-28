pub struct Fearlessness {
    level: i32,
    name: String,
}

impl Fearlessness {
    pub fn new(mut level: i32) -> Fearlessness {
        level = if level < 1 {1} else if level > 3 {3} else {level};
        Fearlessness {
            name: format!("Fearlessness{}", match level {
                ..=1 => " I",
                2 => " II",
                _ => " III",
            }),
            level
        }
    }
}
