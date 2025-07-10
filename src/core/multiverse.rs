use rayon::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use log::{debug, info};

use crate::{core::universe::Universe, util::pluralize::Pluralizer};

const NOT_SO_ABSURD_NUM_OF_UNIVERSES: usize = 1;

pub struct Multiverse {
    _seed: u64,
    universes: Vec<Universe>
}

impl std::fmt::Display for Multiverse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO!
        write!(f, "{}", "Display<Multiverse>: TODO")
    }
}

impl Multiverse {
    pub fn let_there_be_light() -> Self {
        let mut rng = SmallRng::from_os_rng();
        let root_seed = rng.random();
        let mut num_universes = Self::gen_num_of_universes(&mut rng);
        info!("Generating a multiverse with {num_universes} {}", num_universes.pluralize_regular("universe"));
        if cfg!(debug) || cfg!(test) {
            num_universes = NOT_SO_ABSURD_NUM_OF_UNIVERSES;
            debug!("...just kidding — generating {num_universes} {} instead!", num_universes.pluralize_regular("universe"));
        }

        Self {
            _seed: root_seed,
            universes: (0..num_universes)
                        .into_par_iter()
                        .map(move |index| {
                            let universe_seed = root_seed.wrapping_add(index as u64);
                            Universe::big_bang(universe_seed, index)
                        })
                        .collect()
        }
    }

    fn gen_num_of_universes(rng: &mut SmallRng) -> usize {
        // Generate some absurdly (in)sane number …
        let min: u32 = rng.random();
        rng.random_range(min as usize..isize::MAX as usize)
    }

    pub fn universes(&self) -> &Vec<Universe> {
        &self.universes
    }
}

#[cfg(test)]
mod tests {
    use crate::core::multiverse::{Multiverse, NOT_SO_ABSURD_NUM_OF_UNIVERSES};

    #[test]
    fn create_multiverse() {
        let _ = env_logger::try_init();
        let multiverse = Multiverse::let_there_be_light();
        assert!(multiverse.universes.len() == NOT_SO_ABSURD_NUM_OF_UNIVERSES);
    }
}
