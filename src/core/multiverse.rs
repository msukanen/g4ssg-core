use rayon::prelude::*;
use rand::{Rng, SeedableRng, rngs::SmallRng};
use log::{debug, info};

use crate::{core::universe::Universe, util::pluralize::Pluralizer};

const NOT_SO_ABSURD_NUM_OF_UNIVERSES: usize = 1;

pub struct Multiverse {
    seed: u64,
    universes: Vec<Universe>
}

impl Multiverse {
    pub fn let_there_be_light() -> Multiverse {
        let mut rng = SmallRng::from_os_rng();
        let root_seed = rng.random();
        let mut num_universes = Self::gen_num_of_universes();
        info!("Generating a multiverse with {num_universes} {}", num_universes.pluralize_regular("universe"));
        if cfg!(debug) {
            num_universes = NOT_SO_ABSURD_NUM_OF_UNIVERSES;
            debug!("...just kidding — generating {num_universes} {} instead!", num_universes.pluralize_regular("universe"));
        }

        let mut multiverse = Multiverse {
            seed: root_seed,
            universes: (0..num_universes)
                        .into_par_iter()
                        .map(move |index| {
                            let universe_seed = root_seed.wrapping_add(index as u64);
                            Universe::big_bang(universe_seed, index)
                        })
                        .collect()
        };
        multiverse
    }

    fn gen_num_of_universes() -> usize {
        // TODO: generate some (in)sane number…
        NOT_SO_ABSURD_NUM_OF_UNIVERSES
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
