use std::cmp::max;

use dice::DiceExt;

use crate::starsystem::orbital::{OrbitElement, OrbitalInfo};

use super::{size::Size, Planet};

#[derive(Clone)]
pub struct Terrestrial {
    distance: f64,
    size: Size,
    major_moons: Vec<Size>,
    moonlets: i32,
}

impl OrbitalInfo for Terrestrial {
    fn distance(&self) -> f64 {
        self.distance
    }
}

impl Planet for Terrestrial {
    
}

impl Terrestrial {
    pub fn random(distance: f64, size: Size) -> OrbitElement {
        let mut major_moons = vec![];
        let mut moonlets = 0;
        if distance > 0.5 {
            let modifier
             = if distance <= 0.75 {-3}
               else if distance <= 1.5 {-1}
               else {0}
             + match size {
                 Size::Tiny => -2,
                 Size::Small => -1,
                 Size::Large => 1,
                 Size::Medium => 0
             };
             for _ in 0..1.d6() - 4 + modifier {
                major_moons.push(Size::random_moon(size))
             }
            moonlets = max(1.d6() - 2 + modifier, 0)
        }

        OrbitElement::Terrestrial(Terrestrial {
            distance, size,
            major_moons, moonlets,
        })
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn major_moons(&self) -> &Vec<Size> {
        &self.major_moons
    }

    pub fn moonlets(&self) -> i32 {
        self.moonlets
    }
}
