pub mod population;
pub mod evolutionstage;
pub mod designation;
pub mod limits;

use dice::DiceExt;
use rand::Rng;

use crate::{maxof, measurement::massindex::MassIndex, starsystem::orbital::planet::gasgiant::arrangement::GasGiantArrangement};

use self::{evolutionstage::EvolutionStage, limits::{ForbiddenZone, OrbitLimits}, population::Population};

use super::{distance::OrbitalDistance, planet::gasgiant::GasGiant, separation::OrbitalSeparation, OrbitElement};

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
        let mut inward_orbits: Vec<(f64, Option<OrbitElement>)> = vec![];
        let mut outward_orbits: Vec<(f64, Option<OrbitElement>)> = vec![];
        let middle_distance: f64;
        let gga = GasGiantArrangement::random(&orbit_limits);

        if let Some(gga) = gga {
            middle_distance = gga.distance();
            orbits.push((gga.distance(), Some(GasGiant::random(gga))));
        } else {
            middle_distance = orbit_limits.outer(true) / (1.0 + 1.d6() as f64 * 0.05);
        }

        /**
         Generate a random orbital spacing multiplier.
         */
        fn rng_spacing_multiplier() -> f64 {
            match 3.d6() {
                ..=4 => 1.4,
                5|6 => 1.5,
                7|8 => 1.6,
                9..=12 => 1.7,
                13|14 => 1.8,
                15|16 => 1.9,
                17.. => 2.0
            }
        }

        let mut d: f64 = middle_distance;

        //
        // Inwards orbits.
        //
        loop {
            d /= rng_spacing_multiplier();
            if d < orbit_limits.inner(){
                break;
            }
            if !orbit_limits.is_forbidden_distance(d) {
                inward_orbits.push((d, None));//NOTE: 'None' for now, altered later.
            }
        }

        //
        // Outwards orbits.
        //
        d  = middle_distance;
        loop {
            d *= rng_spacing_multiplier();
            if d > orbit_limits.outer(false) {
                break;
            }
            if !orbit_limits.is_forbidden_distance(d) {
                outward_orbits.push((d, None));
            }
        }
        inward_orbits.reverse();
        inward_orbits.extend(orbits);
        inward_orbits.reverse();
        inward_orbits.extend(outward_orbits);
        orbits = inward_orbits;

        //
        // Place gas giants ...
        //
        fn can_place_gg(gga: &GasGiantArrangement, orbit_limits: &OrbitLimits, distance: f64) -> bool {
            match gga {
                GasGiantArrangement::Conventional(_) => distance > orbit_limits.snowline() && 3.d6() < 16,
                GasGiantArrangement::Eccentric(_) => distance <= orbit_limits.snowline() && 3.d6() < 8
                                                  || distance > orbit_limits.snowline() && 3.d6() < 15,
                GasGiantArrangement::Epistellar(_) => distance <= orbit_limits.snowline() && 3.d6() < 7
                                                   || distance > orbit_limits.snowline() && 3.d6() < 15
            }
        }

        let mut gg_orbits = vec![];
        for mut o in orbits {
            match gga {
                None => (),
                Some(gga) => {
                    if can_place_gg(&gga, &orbit_limits, o.0) {
                        o.1 = Some(GasGiant::random(GasGiantArrangement::from((&gga, o.0)), &orbit_limits))
                    }
                }
            }
            gg_orbits.push(o)
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
            orbits: gg_orbits,
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
