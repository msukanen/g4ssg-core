use dicebag::DiceExt;

#[derive(Clone, Copy)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large
}

impl Size {
    pub fn random_gg(inside_snowline_or_first_outside: bool) -> Self {
        let modifier = if inside_snowline_or_first_outside {4} else {0};
        match 3.d6() + modifier {
            ..=10 => Size::Small,
            11..=16 => Size::Medium,
            17.. => Size::Large
        }
    }

    pub fn random_moon(mut size: Self) -> Self {
        fn step_down(size: Size) -> Size {
            match size {
                Size::Tiny  |
                Size::Small => Size::Tiny,
                Size::Medium => Size::Small,
                Size::Large => Size::Medium
            }
        }

        for _ in 0..match 3.d6() {
            ..=11 => 3,
            12..=14 => 2,
            _ => 1
        } { size = step_down(size) }

        size
    }
}
