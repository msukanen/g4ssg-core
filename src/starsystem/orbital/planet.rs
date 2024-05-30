use dice::DiceExt;

use super::OrbitalInfo;

pub mod terrestrial;
pub mod climate;
pub mod gasgiant;
pub mod gravity;

pub trait Planet: OrbitalInfo {}

#[derive(Clone, Copy)]
pub enum Size {
    Tiny,
    Small,
    Medium,
    Large
}

impl Size {
    pub fn random_gg(inside_snowline_or_first_outside: bool) -> Size {
        let modifier = if inside_snowline_or_first_outside {4} else {0};
        match 3.d6() + modifier {
            ..=10 => Size::Small,
            11..=16 => Size::Medium,
            17.. => Size::Large
        }
    }
}

pub enum PlanetType {
    Terrestrial,
    GasGiant,
}
