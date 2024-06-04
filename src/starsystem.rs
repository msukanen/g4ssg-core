use std::collections::HashSet;

use dice::DiceExt;

use crate::config::OutputConfig;

use self::orbital::{distance::OrbitalDistance, separation::OrbitalSeparation, star::{designation::Designation, population::Population, Star}};

pub mod orbital;

pub struct StarSystem {
    html_mode: bool,
    name: String,
    population: Population,
    stars: Vec<Star>,
    designation: HashSet<(Designation, Designation, OrbitalSeparation)>,
}

impl StarSystem {
    fn rng_num_stars(within_open_cluster: bool) -> i32 {
        match 3.d6() + if within_open_cluster {3} else {0} {
            ..=10 => 1,
            11..=15 => 2,
            16.. => 3,
        }
    }

    pub fn random(config: &OutputConfig, name: &str, within_open_cluster: bool) -> StarSystem {
        let population = Population::random();
        let num_stars = Self::rng_num_stars(within_open_cluster);
        let primary_star = Star::random(&population, None, None);
        let mut stars: Vec<Star> = vec![];
        let mut companion_stars = vec![];
        let mut designation = HashSet::new();
        if num_stars > 1 {
            let min_separation = OrbitalSeparation::random(false, 0);
            let min_distance = OrbitalDistance::random(&min_separation);
            // Lets deal next with companion star(s), if any.
            for i in 1..num_stars {
                match i {
                    ..=1 => {
                        designation.insert((Designation::A, Designation::B, min_separation));
                        companion_stars.push(Star::random(&population, Some(primary_star.mass_index()), Some(min_distance)));
                    },
                    2.. => {
                        let mut sep: OrbitalSeparation;
                        loop {
                            sep = OrbitalSeparation::random(true, 0);
                            if sep >= min_separation {
                                break;
                            }
                        }
                        designation.insert((Designation::A, Designation::C, sep));
                        let distance = OrbitalDistance::random(&sep);
                        companion_stars.push(Star::random(&population, Some(primary_star.mass_index()), Some(distance)))
                    },
                };
            }
        }

        stars.push(primary_star);
        stars.extend(companion_stars);

        StarSystem {
            html_mode: config.html_mode,
            name: name.to_string(),
            population,
            stars,
            designation,
        }
    }
}

impl std::fmt::Display for StarSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = vec![];
        for (count, star) in self.stars.iter().enumerate() {
            s.push(format!("♯{} {}\n", count + 1, star))
        }

        write!(f,"\
            Star system: {}\n\
            Stars: {}\n\
              -\n\
            {}",
        self.name,
        self.stars.len(),
        s.join("\n"),
        )
    }
}
