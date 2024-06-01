use std::cmp::min;

use dice::DiceExt;

use crate::{starsystem::orbital::{planet::size::Size, star::population::Population}, util::kind::Kind};

#[derive(Clone, Debug)]
pub enum WorldType {
    Ammonia,
    Chthonian,
    Garden,
    Greenhouse,
    Hadean,
    Ice,
    Ocean,
    Rock,
    Sulfur,
}

impl WorldType {
    pub fn from_blackbody(population: &Population, solar_mass: f64, size: Size, k: f64) -> WorldType {
        match size {
            Size::Tiny =>   if k < 141.0 {if 1.d6() < 3 {Self::Sulfur} else {Self::Ice}} else {Self::Rock},
            Size::Small =>  if k <= 80.0      {Self::Hadean}
                            else if k < 141.0 {Self::Ice}
                            else              {Self::Rock},
            Size::Medium => if k <= 80.0 {Self::Hadean}
                            else if k < 150.0 {Self::Ice}
                            else if k < 231.0 {
                                if solar_mass <= 0.65 {Self::Ammonia} else {Self::Ice}
                            }
                            else if k < 241.0 {Self::Ice}
                            else if k < 321.0 {
                                let byr_diff = min(10, (population.byr() / 0.5) as i32);
                                if 3.d6() + byr_diff >= 18 {Self::Garden} else {Self::Ocean}
                            }
                            else if k < 501.0 {Self::Greenhouse}
                            else {Self::Chthonian},
            Size::Large =>  if k <= 150.0 {Self::Ice}
                            else if k < 231.0 {
                                if solar_mass <= 0.65 {Self::Ammonia} else {Self::Ice}
                            }
                            else if k < 241.0 {Self::Ice}
                            else if k < 321.0 {
                                let byr_diff = min(5, (population.byr() / 0.5) as i32);
                                if 3.d6() + byr_diff >= 18 {Self::Garden} else {Self::Ocean}
                            }
                            else if k < 501.0 {Self::Greenhouse}
                            else {Self::Chthonian}
        }
    }
}

impl std::fmt::Display for WorldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Ammonia => "ammonia",
            Self::Chthonian => "chthonian",
            Self::Garden => "garden",
            Self::Greenhouse => "greenhouse",
            Self::Hadean => "hadean",
            Self::Ice => "ice",
            Self::Ocean => "ocean",
            Self::Rock => "rock",
            Self::Sulfur => "sulfur",
        })
    }
}

impl Kind for WorldType {
    fn kind(&self) -> String {
        match self {
            Self::Ammonia => "ammonium".to_string(),
            Self::Chthonian => "chthonian".to_string(),
            Self::Garden => "garden".to_string(),
            Self::Greenhouse => "greenhouse".to_string(),
            Self::Hadean => "hadean".to_string(),
            Self::Ice => "icy".to_string(),
            Self::Ocean => "oceanic".to_string(),
            Self::Rock => "rocky".to_string(),
            Self::Sulfur => "sulfuric".to_string()
        }
    }
}
