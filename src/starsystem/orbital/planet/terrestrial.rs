pub mod worldtype;
pub mod terratype;

use std::cmp::max;

use dice::{DiceExt, PercentageVariance};
use rand::Rng;
use terratype::TerraType;
use worldtype::WorldType;

use crate::{starsystem::orbital::{star::population::Population, OrbitElement, OrbitalInfo}, util::{distance::{km::Km, Distance, Distanced}, mass::{earth::EarthMass, Mass}}};

use super::{atmosphere::Atmosphere, climate::Climate, density::Density, g, hydrographic::coverage::HydrographicCoverage, size::Size, Planet};

#[derive(Clone)]
pub struct Terrestrial {
    distance: f64,
    terratype: TerraType,
    major_moons: Vec<Size>,
    moonlets: i32,
    atmosphere: Option<Atmosphere>,
    climate: Climate,
    hydrographic: HydrographicCoverage,
    blackbody_k: f64,
    density: Density,
    relative_size: f64,
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

    fn gravity(&self) -> f64 {
        g(&self.density, self.relative_size)
    }

    fn diameter(&self) -> Distance {
        Distance::Km(Km::from(self.relative_size * 7_930.0))
    }

    fn mass(&self) -> Mass {
        Mass::from(EarthMass::from(self.density.value() * self.relative_size * self.relative_size * self.relative_size))
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

        // blackbody avg temp...
        let b = 278.0 * f64::powf(luminosity, 1.0 / 4.0) / distance.sqrt();
        // terratype, obviously...
        let terratype = TerraType::from((size, WorldType::from_blackbody(population, solar_mass, size, b)));
        // hydrographics - may or may not be 'water'...
        let hydrographic = HydrographicCoverage::random(&terratype);
        // blackbody correction...
        let (abf, ghf) = match terratype {
            TerraType::Tiny(WorldType::Ice)        => (0.86, 0.0),
            TerraType::Medium(WorldType::Chthonian)|
            TerraType::Large(WorldType::Chthonian) |
            TerraType::Tiny(WorldType::Rock)       => (0.97, 0.0),
            TerraType::Tiny(WorldType::Sulfur)     => (0.77, 0.0),
            TerraType::Small(WorldType::Hadean)    |
            TerraType::Medium(WorldType::Hadean)   => (0.67, 0.0),
            TerraType::Small(WorldType::Ice)       => (0.93, 0.0),
            TerraType::Small(WorldType::Rock)      => (0.96, 0.0),
            TerraType::Medium(WorldType::Ammonia)  |
            TerraType::Large(WorldType::Ammonia)   => (0.84, 0.2),
            TerraType::Medium(WorldType::Ice)      |
            TerraType::Large(WorldType::Ice)       => (0.86, 0.2),
            TerraType::Medium(WorldType::Ocean)    |
            TerraType::Large(WorldType::Ocean)     |
            TerraType::Medium(WorldType::Garden)   |
            TerraType::Large(WorldType::Garden)    => match hydrographic {
                HydrographicCoverage::Barren => (0.96, 0.0),
                HydrographicCoverage::Percentage(p) => if     p <= 20.0 {(0.95, 0.16)}
                                                            else if p < 51.0 {(0.92, 0.16)}
                                                            else if p < 91.0 {(0.88, 0.16)}
                                                            else             {(0.84, 0.16)}
            },
            TerraType::Medium(WorldType::Greenhouse)|
            TerraType::Large(WorldType::Greenhouse) => (0.77, 2.0),
            _ => panic!("Unspecified terratype selection: {:?}", terratype)
        };
        // atmosphere, if any?
        let atmosphere = Atmosphere::potential_mass(&terratype);
        let blackbody_correction = if let Some(a) = &atmosphere {
            abf * (1.0 + a * ghf)
        } else {
            abf
        };
        // climate...
        let climate = Climate::from(&terratype);
        // density...
        let density = Density::from(&terratype);
        // Earth-relative size...
        let szc = match terratype {
            TerraType::Tiny(_) => (0.004, 0.024),
            TerraType::Small(_) => (0.024, 0.03),
            TerraType::Medium(_) => (0.03, 0.065),
            TerraType::Large(_) => (0.065, 0.091)
        };
        let modifier = blackbody_correction * climate.avg_temperature() / density.value();
        let relative_size = rand::thread_rng().gen_range(modifier * szc.0..=modifier * szc.1).delta(5);
        let atmosphere = Atmosphere::random(&terratype, g(&density, relative_size));

        OrbitElement::Terrestrial(Terrestrial {
            distance, terratype,
            major_moons, moonlets,
            atmosphere, climate, hydrographic,
            blackbody_k: blackbody_correction * climate.avg_temperature(),
            density, relative_size,
        })
    }

    /**
     Get the planet's climate.
     */
    pub fn climate(&self) -> &Climate {
        &self.climate
    }
}
