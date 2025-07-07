use log::info;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::{core::galaxy::Galaxy, util::pluralize::Pluralizer};

const NOT_SO_ABSURD_NUM_OF_GALAXIES: usize = 1;

pub struct Universe {
    seed: u64,
    galaxies: Vec<Galaxy>
}

impl Universe {
    /// Generates a very big bang… which creates a brand new universe to explore and exploit!
    pub fn big_bang(root_seed: u64, index: usize) -> Self {
        let mut rng = SmallRng::seed_from_u64(root_seed);
        let root_seed = rng.random();
        let num_of_galaxies = Self::gen_num_of_galaxies();
        info!("Generating a universe with {num_of_galaxies} {}", num_of_galaxies.pluralize("galaxy", "galxies"));
        
        Self {
            seed: root_seed,
            galaxies: (0..num_of_galaxies)
                        .into_par_iter()
                        .map(move |index| {
                            let galaxy_seed = root_seed.wrapping_add(index as u64);
                            Galaxy::new(galaxy_seed, index, None)
                        })
                        .collect()
        }
    }

    /// Gets the number of (currently existing) galaxies in the particular universe.
    pub fn num_galaxies(&self) -> usize {
        self.galaxies.len()
    }

    /// Generates an absurdly high random number…
    /// as there is absurdly lot of galaxies out there afterall.
    fn gen_num_of_galaxies() -> usize {
        // TODO: an absurd number between 50 billion and 10 trillion, at least.
        NOT_SO_ABSURD_NUM_OF_GALAXIES
    }
}


#[cfg(test)]
mod tests {
    use crate::core::universe::{Universe, NOT_SO_ABSURD_NUM_OF_GALAXIES};

    #[test]
    fn create_universe() {
        let _ = env_logger::try_init();
        let universe = Universe::big_bang(0, 0);
        assert!(universe.galaxies.len() == NOT_SO_ABSURD_NUM_OF_GALAXIES);
    }
}
