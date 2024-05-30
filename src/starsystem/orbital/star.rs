pub mod population;
pub mod evolutionstage;
pub mod designation;
pub mod limits;

use dice::DiceExt;
use rand::Rng;

use crate::{life::sex::arrangement, maxof, measurement::massindex::MassIndex};

use self::{evolutionstage::EvolutionStage, limits::{ForbiddenZone, OrbitLimits}, population::Population};

use super::{distance::OrbitalDistance, planet::{gasgiant::{GasGiant, GasGiantArrangement}, Planet}, separation::OrbitalSeparation, OrbitElement};

pub struct Star {
    population: Population,
    companion: Option<Box<Star>>,
    distance: Option<OrbitalDistance>,
    mass_index: i32,
    mass: f64,
    temperature: f64,
    evolution: EvolutionStage,
    luminosity: f64,
    radius: f64,
    orbit_limits: OrbitLimits,
    orbits: Vec<(f64, Option<OrbitElement>)>,
}

impl Star {
    pub fn random(population: &Population, companion_to_mass_index: Option<i32>, distance: Option<OrbitalDistance>) -> Star {
        let mass_index = if let Some(ctmi) = companion_to_mass_index {
            (ctmi + (1.d6() - 1).d6()).clamp_mass_index()
        } else {
            Self::rng_mass_index()
        };
        let mut mass = mass_index.solar_mass();
        let temperature: f64;
        let agespans = mass_index.solar_agespans();
        let evolution = EvolutionStage::from((population, agespans));
        let luminosity: f64;
        let lmin = mass_index.solar_lmin();
        let lmax = mass_index.solar_lmax();
        let initial_luminosity = match lmax {
            None => lmin,
            Some(lmax) => lmin + ((population.byr() / agespans.0.unwrap()) * (lmax - lmin))
        };
        match evolution {
            EvolutionStage::M => {
                luminosity = initial_luminosity;
                temperature = mass_index.solar_temperature();
            },
            EvolutionStage::S => {
                temperature = mass_index.solar_temperature() - ((population.byr() - agespans.0.unwrap()) / agespans.1.unwrap() * (mass_index.solar_temperature() - 4_800.0));
                luminosity = lmax.unwrap();
            },
            EvolutionStage::G => {
                temperature = 3_000.0 + 200.0 * (2.d6() - 2) as f64;
                luminosity = lmax.unwrap() * 25.0;
            },
            EvolutionStage::D => {
                temperature = mass_index.solar_temperature() * 12.5;
                mass = 0.9 + 0.05 * (2.d6() - 2) as f64;
                luminosity = rand::thread_rng().gen_range(0.000001..=0.001);
            }
        }
        let radius = match evolution {
            EvolutionStage::D => 0.0,
            _ => (155_000.0 * luminosity.sqrt()) / (temperature * temperature)
        };

        let mut companion: Option<Box<Star>> = None;
        let mut forbidden_zone = None;
        if let Some(ref distance) = distance {
            if distance.separation() == &OrbitalSeparation::Distant && 3.d6() >= 11 {
                forbidden_zone = Some(ForbiddenZone::from((distance.min(), distance.max())));
                companion = Some(Box::new(Star::random(population, Some(mass_index), Some(OrbitalDistance::random(&OrbitalSeparation::random(false, -6))))))
            }
        }

        // Determine inner limit, outer limit and snowline (in AU) alongside forbidden zone.
        let orbit_limits = OrbitLimits::from((
            maxof!(0.1 * mass, 0.01 * luminosity.sqrt()),
            40.0 * mass,
            4.85 * initial_luminosity.sqrt(),
            forbidden_zone,
        ));

        let mut orbits: Vec<(f64, Option<OrbitElement>)> = vec![];

        if let Some(gga) = GasGiantArrangement::random(&orbit_limits) {
            orbits.push((gga.distance(), Some(GasGiant::random(gga))));
        }

        Star {
            population: *population,
            companion,
            distance,
            mass_index, mass,
            temperature,
            evolution,
            luminosity,
            radius,
            orbit_limits,
            orbits,
        }
    }

    /**
     Generate a random mass index.
     */
    fn rng_mass_index() -> i32 {
        match 3.d6() {
            ..=3 => match 3.d6() {
                ..=10 => 0,
                11.. => 1
            },
            4 => match 3.d6() {
                ..=8 => 2,
                9..=11 => 3,
                12.. => 4
            },
            5 => match 3.d6() {
                ..=7 => 5,
                8..=10 => 6,
                11|12 => 7,
                13.. => 8
            },
            6 => match 3.d6() {
                ..=7 => 9,
                8|9 => 10,
                10 => 11,
                11|12 => 12,
                13.. => 13
            },
            7 => match 3.d6() {
                ..=7 => 14,
                8|9 => 15,
                10 => 16,
                11|12 => 17,
                13.. => 18
            },
            8 => match 3.d6() {
                ..=7 => 19,
                8|9 => 20,
                10 => 21,
                11|12 => 22,
                13.. => 23
            },
            9 => match 3.d6() {
                ..=8 => 24,
                9..=11 => 25,
                12.. => 26
            },
            10 => match 3.d6() {
                ..=8 => 27,
                9..=11 => 28,
                12.. => 29
            },
            11 => 30,
            12 => 31,
            13 => 32,
            14.. => 33
        }
    }

    pub fn mass_index(&self) -> i32 {
        self.mass_index
    }
}
