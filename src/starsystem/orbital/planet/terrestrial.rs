pub mod worldtype;
pub mod terratype;

use std::cmp::max;

use dice::DiceExt;
use terratype::TerraType;
use worldtype::WorldType;

use crate::starsystem::orbital::{star::population::Population, OrbitElement, OrbitalInfo};

use super::{atmosphere::Atmosphere, size::Size, Planet};

#[derive(Clone)]
pub struct Terrestrial {
    distance: f64,
    terratype: TerraType,
    major_moons: Vec<Size>,
    moonlets: i32,
    atmosphere: Option<Atmosphere>,
}

impl OrbitalInfo for Terrestrial {
    fn distance(&self) -> f64 {
        self.distance
    }
}

impl Planet for Terrestrial {
    fn size(&self) -> Size {
        match self.terratype {
            TerraType::Small(_) => Size::Small,
            TerraType::Medium(_) => Size::Medium,
            TerraType::Large(_) => Size::Large,
            TerraType::Tiny(_) => Size::Tiny
        }
    }

    fn major_moons(&self) -> &Vec<Size> {
        &self.major_moons
    }

    fn moonlets(&self) -> i32 {
        self.moonlets
    }

    fn atmosphere(&self) -> Option<Atmosphere> {
        self.atmosphere.clone()
    }
}

impl Terrestrial {
    /**
     Generate a random "terrestrial" planet.
     */
    pub fn random(population: &Population, solar_mass: f64,  luminosity: f64, distance: f64, size: Size) -> OrbitElement {
        // sort out the moons...
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

        let b = 278.0 * f64::powf(luminosity, 1.0 / 4.0) / distance.sqrt();
        let terratype = TerraType::from((size, WorldType::from_blackbody(population, solar_mass, size, b)));
        let atmosphere = Atmosphere::random(&terratype);

        OrbitElement::Terrestrial(Terrestrial {
            distance, terratype,
            major_moons, moonlets,
            atmosphere,
        })
    }
}
