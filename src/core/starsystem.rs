use std::sync::Arc;

use dicebag::DiceExt;
use log::{debug, info};
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{core::{star::{BaseStar, Star}, stellar_age::StellarAge}, util::pluralize::Pluralizer};

pub struct StarSystem {
    seed: u64,
    designation: String,
    stars: Vec<Star>,
    age: Arc<StellarAge>, // This will be propagated to all the contained [Star].
}

impl StarSystem {
    /// Generate a new star system with 0+ stars (or other major celestial
    /// objects, e.g. black holes, etc.).
    /// 
    /// ### Params
    /// * **icoc** — is the system "in core or a cluster"?
    /// * **designation** — designation, name, etc. of the system.
    pub fn new<'a>(root_seed: u64, index: usize, icoc: bool, designation: Option<&str>) -> Self {
        let r = 3.d(6) + if icoc {3} else {0};
        // The initial number of stars. This may change as the system
        // evolves further.
        let num_stars: usize = match r {
            3..=10 => 1,
            11..=15 => 2,
            _ => 3
        };
        info!("Creating star system {{#{}}} with {num_stars} {}", index + 1, num_stars.pluralize_regular("star"));
        debug!("Num.of stars {num_stars}");

        let age = Arc::new(StellarAge::new());
        debug!("Stellar age {age}");
        let mut rng = SmallRng::seed_from_u64(root_seed);
        let root_seed: u64 = rng.random();// new root seed.

        // Base stars have to be created linearly before they can branch out
        // as standalone individuals.
        let mut base_stars = vec![];
        let mut current_evo_index = 0;

        // TODO: as of now, star generation goes linearly [larger → smaller]. But in real life,
        //       deviations to that exist (our own planetary system as the very real example).
        //       Sort that out when the core otherwise functions as desired.
        for index in 0..num_stars {
            let star_seed = root_seed.wrapping_add(index as u64);
            let base_star = BaseStar::new(star_seed, index.into(), current_evo_index, designation);
            current_evo_index = base_star.evolution_index();
            base_stars.push(base_star);
        }

        let (plurs, plura) = if base_stars.len() > 1 {("s", "")} else {("", "a ")};
        info!("Evolving BaseStar{plurs} into fully {plura}fledged Star{plurs}");
        let stars = base_stars.into_par_iter()
                .map(|b| {
                    b.at_age(Arc::clone(&age))
                })
                .collect();

        StarSystem {
            seed: root_seed,
            //TODO: random designation if None offered.
            designation: designation.map(String::from).unwrap_or_else(Self::gen_random_designation),
            age,
            stars,
        }
    }

    /// Get the number of (currently existing) stars (or other major celestial
    /// objects) in the system.
    pub fn num_stars(&self) -> usize {
        self.stars.len()
    }

    /// Generate some random designation/name.
    /// TODO: implement…
    fn gen_random_designation() -> String {
        "<TODO:designation>".to_string()
    }
}
