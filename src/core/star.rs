//! BaseStar and Star live here!
use std::sync::Arc;
use log::{debug, info};
use msuk_scifi::unit::{distance::au::Au, temperature::k::K};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use dicebag::{DiceExt, FixedNumberVariance, PercentageVariance};

use crate::core::{stellar_age::{ByrExt, StellarAge}, stellar_evo::{StellarEvolution, StellarEvolutionSequence, StellarRemnantClassification, EVOLUTIONS}, stellar_mass::gen_stellar_mass};

#[derive(Clone, PartialEq)]
pub enum OrderInSystem {
    Primary,
    Secondary,
    Tertiary,
    Quaternary,
    Quinary,
    Senary,
    Septenary,
}

impl From<usize> for OrderInSystem {
    /// Creates [OrderInSystem] from e.g. index number.
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Primary,
            1 => Self::Secondary,
            2 => Self::Tertiary,
            3 => Self::Quaternary,
            4 => Self::Quinary,
            5 => Self::Senary,
            _/* 6 */ => Self::Septenary,
            // TODO: 7,8,9,10,...?
        }
    }
}
impl From<u32> for OrderInSystem { fn from(value: u32) -> Self { OrderInSystem::from(value as usize)}}
impl From<i32> for OrderInSystem { fn from(value: i32) -> Self { OrderInSystem::from(if value < 0 {0} else {value as usize})}}
impl std::fmt::Display for OrderInSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Primary => "primary",
            Self::Secondary => "secondary",
            Self::Tertiary => "tertiary",
            Self::Quaternary => "quaternary",
            Self::Quinary => "quinary",
            Self::Senary => "senary",
            Self::Septenary => "septenary",
        })
    }
}

pub struct BaseStar {
    seed: u64,
    rng: SmallRng,
    designation: String,
    order_in_system: OrderInSystem,
    evo_index: usize,
    evo: &'static StellarEvolution,
    mass: f64,
}

/// Here be a star, yes.
pub struct Star {
    seed: u64,
    rng: SmallRng,
    designation: String,
    order_in_system: OrderInSystem,
    mass: f64,
    sequence: StellarEvolutionSequence,
    evo: &'static StellarEvolution,
    age: Arc<StellarAge>,
    luminosity: f64,
    temperature: K,
    radius: Au,
    // inner, outer, snow line:
    orbital_zones: (Au, Au, Au),
    forbidden_zones: Vec<(Au, Au)>,
}

impl BaseStar {
    /// Generates a shiny star (usually…).
    /// 
    /// # Arguments
    /// 
    /// * `root_seed`— parent-defined RNG root seed.
    /// * `order_in_system`— 
    /// * `non_primary_evo_index`— if `order_in_system` is not [OrderInSystem::Primary],
    ///                            this value will be used for evolution ladder stepping.
    ///                            Otherwise the value is utterly ignored.
    /// * `designation`— name, designation, etc. of the star. If None, a random name will be assigned.
    /// 
    pub fn new(root_seed: u64, order_in_system: OrderInSystem, non_primary_evo_index: usize, designation: Option<&str>) -> Self {
        let mut rng = SmallRng::seed_from_u64(root_seed);
        // If star isn't primary, we step down the evolution ladder toward (potentially) smaller star size.
        let (mass, eidx, evo) =
            if order_in_system != OrderInSystem::Primary {
                let mut evo_index = non_primary_evo_index;
                let evo_delta = 1.d6() - 1;
                if evo_delta > 0 {
                    // Negative vector index is not legal, so… zero out to point to the smallest size listed.
                    evo_index = evo_index.saturating_sub(evo_delta.d6());
                }

                info!("Creating base star of EVO #{evo_index}");
                // EVOLUTIONS is ~ensured~ to have something and thus we can just unwrap() it.
                let evo = EVOLUTIONS.get(evo_index).unwrap();
                (evo.rel_mass.upto_delta(0.05), evo_index, evo)
            } else {
                let mass = gen_stellar_mass();
                let (eidx, evo) = StellarEvolution::find_by_mass(mass);
                info!("Creating base star of EVO#{eidx}");
                (mass.upto_delta(0.05), eidx, evo)
            };
        
        BaseStar {
            seed: rng.random(), rng,
            //TODO: random designation if None offered.
            designation: designation.map(String::from).unwrap_or_else(Self::gen_random_designation),
            evo_index: eidx,
            evo,
            order_in_system, mass,
        }
    }

    /// Get evolution index.
    /// 
    /// NOTE: mainly useful only for
    ///       [StarSystem::new(…)][`crate::core::starsystem::StarSystem::new`].
    pub fn evolution_index(&self) -> usize {
        self.evo_index
    }

    fn gen_random_designation() -> String {
        "<TODO:designation>".to_string()
    }

    #[cfg(test)]
    fn adjust_evo_index(&mut self, evo_index: usize) {
        use log::debug;

        let _ = env_logger::try_init();
        debug!("Adjusting evolution index from {} to {evo_index}", self.evo_index);
        self.evo_index = evo_index;
        self.evo = &EVOLUTIONS[self.evo_index];
        self.mass = self.evo.rel_mass.upto_delta(0.05);
    }

    /// Generate a [Star] at specified age.
    /// 
    /// NOTE: this consumes the [BaseStar].
    /// 
    /// # Arguments
    /// * `self`— [BaseStar] (**mut**, consumed).
    /// * `age`— Star system's [age][StellarAge].
    pub fn at_age(mut self, age: Arc<StellarAge>) -> Star {
        info!("Evolving a base star into fully fledged star.");
        debug!("Mass ({}): {}", self.evo.rel_mass, self.mass);

        let sequence = self.evo.determine_sequence(&age);/* 1st */
        let luminosity = Star::determine_luminosity(&mut self.rng, self.evo, &age, &sequence);
        let temperature = Star::determine_temperature(&mut self.rng, self.evo, &age, &sequence);
        let radius = Star::determine_radius(&sequence, self.mass, luminosity, &temperature);

        let lsqrt = luminosity.sqrt();
        let orbital_zones = (
            Au::from((0.1 * self.mass).max(0.01 * lsqrt)),
            Au::from( 40.0 * self.mass),
            Au::from( 4.85 * lsqrt)
        );

        Star {
            // Bring in [BaseStar] data first and foremeost…
            seed: self.seed,
            rng: self.rng,
            designation: self.designation,
            order_in_system: self.order_in_system,
            mass: self.mass,
            evo: self.evo,
            // … and then the rest:
            sequence,
            age,
            luminosity,
            temperature,
            radius,
            orbital_zones,
            forbidden_zones: vec![],// these will be decided later.
        }
    }
}

impl Star {
    /// Determine a [star's][Star] luminosity (with up to ±10% variance).
    /// 
    /// # Arguments
    /// * `rng`— reference to the [star's][Star] own rng.
    /// * `evo`— reference to the [star's][Star] core evo data.
    /// * `age`— the [star's][Star] / [star system's][StarSystem] age.
    /// * `sequence`— the [star's][Star] stellar sequence.
    /// 
    /// # Returns
    /// A luminosity value.
    fn determine_luminosity(rng: &mut SmallRng, evo: &'static StellarEvolution, age: &Arc<StellarAge>, sequence: &StellarEvolutionSequence) -> f64 {
        let lum = match sequence {
            // Stellar remnants have barely any visible spectrum luminosity to speak of, e.g. rarely higher than 0.001 for a WD.
            StellarEvolutionSequence::Remnant(_) => rng.random::<f64>() / 1_000.0,

            StellarEvolutionSequence::MainSequence => match (evo.span_m, evo.luminosity_max) {
                (Some(span_m), Some(lum_max)) => {
                    let lifespan_progress = age.years / span_m;
                    evo.luminosity_min + (lifespan_progress * (lum_max - evo.luminosity_min))
                },
                // Fall back to minimum if no M-span and/or max. luminosity defined.
                _ => evo.luminosity_min
            }

            // NOTE: (sub)giants ~always~ have lum.max. defined, thus safe to unwrap without testing.
            StellarEvolutionSequence::Subgiant => evo.luminosity_max.unwrap(),

            // NOTE: (sub)giants ~always~ have lum.max. defined, thus safe to unwrap without testing.
            StellarEvolutionSequence::Giant(_) => evo.luminosity_max.unwrap() * 25.0
        };
        // ±10%:
        let d_lum = lum.delta(10);
        debug!("Luminosity {lum} → {d_lum}");
        d_lum
    }

    /// Determine a [star's][Star] surface temperature (with up to ±100K variance).
    /// 
    /// # Arguments
    /// * `rng`— reference to the [star's][Star] own rng.
    /// * `evo`— reference to the [star's][Star] core evo data.
    /// * `age`— the [star's][Star] / [star system's][StarSystem] age.
    /// * `sequence`— the [star's][Star] stellar sequence.
    /// 
    /// # Returns
    /// Kelvins.
    fn determine_temperature(rng: &mut SmallRng, evo: &'static StellarEvolution, age: &Arc<StellarAge>, sequence: &StellarEvolutionSequence) -> K {
        let k = match sequence {
            StellarEvolutionSequence::MainSequence =>
                (evo.surface_temperature as f64).upto_delta(100.0).round(),
            StellarEvolutionSequence::Subgiant => {
                // T = (M - ((A / S) × (M - 4,800))) ± 100K
                let m = evo.surface_temperature as f64;
                let a = age.as_byr() - evo.span_m.unwrap();
                let s = evo.span_s.unwrap();
                (m - (a / s) * (m - 4_800.0)).upto_delta(100.0).round()
            },
            StellarEvolutionSequence::Giant(_) => {
                (rng.random::<f64>() * 3_000.0 + 2_000.0).upto_delta(100.0).round()
            },
            StellarEvolutionSequence::Remnant(_) => {
                debug!("Assigning 0K as 'effective temperature' for a stellar remnant");
                0.0
            }
        }
        .into();
        debug!("Temperature {k}");
        k
    }

    /// Determine star radius.
    ///
    // For normal stars, radius and temperature are linked through luminosity. For remnants,
    // the radius is dictated by much more extreme physics:
    // * White Dwarfs: The radius is determined by electron degeneracy pressure and
    //   is inversely related to its mass (more massive ones are smaller).
    // * Neutron Stars: The radius is set by neutron degeneracy pressure.
    // * Black Holes: The radius is its Schwarzschild radius, which depends only on its mass.
    fn determine_radius(sequence: &StellarEvolutionSequence, mass: f64, luminosity: f64, temperature: &K) -> Au {
        let r = match sequence {
            StellarEvolutionSequence::Remnant(x) => Star::determine_remnant_radius(x),
            _ =>// R = (155,000 × √L) / T²
                Au::from((155_000.0 * luminosity.sqrt()) / (temperature.value() * temperature.value()))
        };
        debug!("Radius {r}");
        r
    }

    fn determine_remnant_radius(_classification: &StellarRemnantClassification) -> Au {
        // TODO: the hard math …
        0.000000001.into()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use log::debug;

    use crate::core::{star::BaseStar, stellar_age::{StellarAge, StellarPopulation}};

    fn make_base_star() -> BaseStar {
        BaseStar::new(0, 0.into(), 0, None)
    }

    #[test]
    fn create_primary_base_star() {
        let _ = env_logger::try_init();
        let base_star = make_base_star();
        assert!(base_star.mass >= 0.05 && base_star.mass <= 2.05);
    }

    #[test]
    fn adjust_base_star_evo_index() {
        let _ = env_logger::try_init();
        let mut base_star = make_base_star();
        debug!("BaseStar old mass {}", base_star.mass);
        base_star.adjust_evo_index(2);
        debug!("BaseStar new mass {}", base_star.mass);
    }

    #[test]
    fn create_star_from_base_star() {
        let _ = env_logger::try_init();
        let base_star = make_base_star();
        let age = 5_000_000_000.0;
        debug!("Generating a star at {age} years of age…");
        let age: Arc<StellarAge> = StellarAge::from((StellarPopulation::OldI, age)).into();
        let star = base_star.at_age(Arc::clone(&age));
        debug!("Star life stage {}", star.sequence);
    }
}
