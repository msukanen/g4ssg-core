use dicebag::DiceExt;
use log::{debug, info};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{core::starsystem::StarSystem, util::pluralize::Pluralizer};

const NOT_SO_ABSURD_NUM_OF_STAR_SYSTEMS: usize = 10;
const STAR_SYSTEM_NUM_GENERATION_SKEW: f64 = 15.0;

pub struct Galaxy {
    seed: u64,
    designation: String,
    star_systems: Vec<StarSystem>
}

impl std::fmt::Display for Galaxy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO!
        write!(f, "{}", "Display<Galaxy>: TODO")
    }
}

impl Galaxy {
    /// Generates a shiny, brand new galaxy!
    /// 
    /// # Params
    /// * `index` — [logging only][log] index of the galaxy generated.
    /// * `designation` — name, designation, etc. of the galaxy.
    pub fn new(root_seed: u64, index: usize, designation: Option<&str>) -> Self {
        let mut rng = SmallRng::seed_from_u64(root_seed);
        let root_seed: u64 = rng.random();
        
        let mut num_star_systems = Self::absurd_scale_random(STAR_SYSTEM_NUM_GENERATION_SKEW);
        info!("Generating galaxy {{#{}}} with {num_star_systems} star {}", index+1, num_star_systems.pluralize_regular("system"));
        if cfg!(not(release)) {
            num_star_systems = NOT_SO_ABSURD_NUM_OF_STAR_SYSTEMS;
            debug!("...just kidding — generating {num_star_systems} star {} instead!", num_star_systems.pluralize_regular("system"));
        }
        
        Self {
            seed: root_seed,
            //TODO: random designation if None offered.
            designation: designation.unwrap_or("<TODO:designation>").to_string(),
            star_systems: (0..num_star_systems)
                            .into_par_iter()
                            .map(move |index| {
                                let system_seed = root_seed.wrapping_add(index as u64);
                                let icoc = 1.d6() <= 1;
                                StarSystem::new(system_seed, index, icoc, None)
                            })
                            .collect()
        }
    }

    /// Get the number of (currently existing) star systems in the particular
    /// galaxy in question.
    pub fn num_systems(&self) -> usize {
        self.star_systems.len()
    }

    pub fn absurd_scale_random(skew: f64) -> usize {
        // TODO: fix this - leans too much to low side... Alter either how this works or play with skew value?
        const MIN_VAL: u128 = 1_000;
        const MAX_VAL: u128 = 100_000_000_000_000_000;
        let log_min = (MIN_VAL as f64).ln();
        let log_max = (MAX_VAL as f64).ln();
        let mut r = rand::rng();
        let wt: f64 = (r.random_range(0.0..1.0) as f64).powf(skew);
        ((log_min + (log_max - log_min)) * wt).exp() as usize
    }
}

#[cfg(test)]
mod tests {
    use env_logger;
    use log::{debug, info};
    use crate::util::probability_density::{find_k, sample_power_law};

    #[test]
    fn testing_100_million_average() {
        let _ = env_logger::try_init();

        const A: f64 = 1_000.0;
        debug!("a = {A}");
        
        const B: f64 = 100_000_000_000_000_000.0;
        debug!("b = {B}");
        
        const TARGET_MEAN: f64 = 100_000_000.0;
        debug!("t = {TARGET_MEAN}");
        
        let k = find_k(A, B, TARGET_MEAN);
        debug!("k = {k}");
        
        let mut t = 0.0;
        #[cfg(feature = "deeptest")]
        const SAMPLES: usize = 100_000_000;
        #[cfg(not(feature = "deeptest"))]
        const SAMPLES: usize = 50;

        for x in 1..=SAMPLES {
            let r = sample_power_law(A, B, k);
            t += r;
            debug!("{x} :: A={A} B={B} → {r} with k of {k}");
        }
        info!("{SAMPLES} samples in range {A} … {B} with overall average of {}", t / SAMPLES as f64);
    }
}
