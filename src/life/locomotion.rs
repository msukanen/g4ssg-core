use std::collections::HashSet;

use dice::{low, DiceExt, HiLo};

use super::{habitat::{Habitat, LandHabitat, WaterHabitat}, trophiclevel::{Carnivore, Herbivore, TrophicLevel}};

#[derive(Clone, Copy, PartialEq)]
pub enum FlightMode {
    Buoyant,
    Gliding,
    Rocket,
    SolarSail,
    Winged,
}

//NOTE: immobile creatures have Option<Locomotion> = None
#[derive(Clone, Copy, PartialEq)]
pub enum Locomotion {
    Climbing(bool),
    Digging,
    Flight(FlightMode),
    Floating,
    Sailing,
    Sliding,
    Slithering,
    Swimming,
    Special,
    Walking,
}

impl Locomotion {
    /**
     Generate locomotion type(s), if any.

     Unless result is **None**, the first item in the returned vector is the *primary* locomotion method.
     Secondary/tertiary modes of locomotion may or may not be present.
     */
    pub fn random(habitat: &Habitat, trophiclevel: &HashSet<TrophicLevel>, gasgiant: bool) -> Option<Vec<Locomotion>> {
        let mut locomotions = vec![];
        let modifier =
            if trophiclevel.contains(&TrophicLevel::Carnivore(Carnivore::Chasing)) ||
               trophiclevel.contains(&TrophicLevel::Carnivore(Carnivore::Chasing)) ||
               trophiclevel.contains(&TrophicLevel::Carnivore(Carnivore::Pouncing))||
               trophiclevel.contains(&TrophicLevel::Omnivore)                      ||
               trophiclevel.contains(&TrophicLevel::Scavenger)                     ||
               trophiclevel.contains(&TrophicLevel::Herbivore(Herbivore::Gathering))
                 {1}
            else {0};
        
        if gasgiant {
            //
            // Return early for gas giants; habitat doesn't matter there.
            //
            return Some(vec![match 2.d6() + modifier {
                ..=5 => Self::Swimming,
                6..=8 => Self::Flight(FlightMode::Winged),
                _ => Self::Flight(FlightMode::Buoyant)
            }]);
        }

        match habitat {
            //
            // Land habitats...
            //
            Habitat::Land(h) => match h {
                LandHabitat::Arctic => match 2.d6() + modifier {
                    ..=2 => return None,
                    3|4 => locomotions.push(Self::Slithering),
                    5|6 => {
                        let mode = Self::Swimming;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    7 => {
                        let mode = Self::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    8|9 => locomotions.push(Self::Walking),
                    10|11 => locomotions.push(Self::Flight(FlightMode::Winged)),
                    _ => locomotions.push(Self::Special)
                },
                LandHabitat::Desert => match 2.d6() + modifier {
                    ..=2 => return None,
                    3|4 => locomotions.push(Self::Slithering),
                    5 => {
                        let mode = Self::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    6..=8 => locomotions.push(Self::Walking),
                    9..=11 => {
                        let mode = Self::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(Self::Special)
                },
                LandHabitat::IslandAndBeach => match 2.d6() + modifier {
                    ..=2 => return None,
                    3|4 => locomotions.push(Self::Slithering),
                    5 => {
                        let mode = Self::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    6|7 => locomotions.push(Self::Walking),
                    8 => {
                        let mode = Self::Climbing(true);
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    9 => {
                        let mode = Self::Swimming;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    10|11 => {
                        let mode = Self::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(Self::Special)
                },
                LandHabitat::Mountain => match 2.d6() + modifier {
                    ..=2 => return None,
                    3|4 => locomotions.push(Self::Slithering),
                    5 => {
                        let mode = Self::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    6|7 => {
                        let mode = Self::Walking;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    8 => {
                        let mode = Self::Climbing(false);
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    9..=11 => {
                        let mode = Self::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(Self::Special)
                },
                LandHabitat::Plains => match 2.d6() + modifier {
                    ..=2 => return None,
                    3|4 => locomotions.push(Self::Slithering),
                    5 => {
                        let mode = Self::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    6..=8 => locomotions.push(Self::Walking),
                    9..=11 => {
                        let mode = Self::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(Self::Special)
                },
                LandHabitat::Swampland => match 2.d6() + modifier {
                    ..=2 => return None,
                    3..=5 => {
                        let mode = Self::Swimming;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    6 => locomotions.push(Self::Slithering),
                    7 => {
                        let mode = Self::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    8 => locomotions.push(Self::Walking),
                    9 => {
                        let mode = Self::Climbing(1.d2().lo());
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    10|11 => {
                        let mode = Self::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(Self::Special)
                },
                LandHabitat::Woodlands |
                LandHabitat::Jungle => match 2.d6() + modifier {
                    ..=2 => return None,
                    3|4 => locomotions.push(Self::Slithering),
                    5 => {
                        let mode = Self::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    6|7 => locomotions.push(Self::Walking),
                    8|9 => {
                        let mode = Self::Climbing(true);
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    10|11 => {
                        let mode = Self::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(Self::Special)
                }
            },
            //
            // Water habitats...
            //
            Habitat::Water(w) => match w {
                WaterHabitat::Banks |
                WaterHabitat::OpenOcean => match 2.d6() + modifier {
                    ..=3 => return None,
                    4 => locomotions.push(Self::Floating),
                    5 => locomotions.push(Self::Sailing),
                    6..=8 => locomotions.push(Self::Swimming),
                    9..=11 => {
                        let mode = Self::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(Self::Special)
                },
                WaterHabitat::Reef |
                WaterHabitat::DeepOceanVents => match 2.d6() + modifier {
                    ..=5 => return None,
                    6 => locomotions.push(Self::Floating),
                    7 => {
                        let mode = Self::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    8|9 => {
                        let mode = Self::Walking;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(Self::Swimming)
                },
                WaterHabitat::TropicalLagoon => match 2.d6() + modifier {
                    ..=4 => return None,
                    5 => locomotions.push(Self::Floating),
                    6 => {
                        let mode = Self::Slithering;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    7 => {
                        let mode = Self::Walking;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    8 => {
                        let mode = Self::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    9 => locomotions.push(Self::Swimming),
                    10|11 => locomotions.push(Self::Flight(FlightMode::Winged)),
                    _ => locomotions.push(Self::Special)
                },
                WaterHabitat::FreshWaterLakes |
                WaterHabitat::SaltWaterSea => match 2.d6() + modifier {
                    ..=3 => return None,
                    4 => locomotions.push(Self::Floating),
                    5 => {
                        let mode = Self::Walking;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    6 => {
                        let mode = Self::Slithering;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    7..=9 => locomotions.push(Self::Swimming),
                    10|11 => {
                        let mode = Self::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(Self::Special)
                },
                WaterHabitat::RiverOrStream => match 2.d6() + modifier {
                    ..=3 => return None,
                    4 => locomotions.push(Self::Floating),
                    5 => {
                        let mode = Self::Slithering;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    6 => {
                        let mode = Self::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    7 => {
                        let mode = Self::Walking;
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    8|9 => locomotions.push(Self::Swimming),
                    10|11 => {
                        let mode = Self::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Self::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(Self::Special)
                }
            },
            Habitat::Space => match 2.d6() + modifier {
                //TODO: expand on list of Habitat::Space locomotion modes.
                ..=6 => return None,
                7..=11 => locomotions.push(Self::Flight(FlightMode::SolarSail)),
                12.. => locomotions.push(Self::Flight(FlightMode::Rocket))
            },
            //
            // Exotica always uses a "Special" locomotion mode.
            //
            Habitat::Exotica => locomotions.push(Self::Special)
        }
        
        Some(locomotions)
    }

    fn random_non_primary(habitat: &Habitat, mode: Locomotion) -> Vec<Locomotion> {
        Self::random_2or3(habitat, mode, false)
    }

    fn random_2or3(habitat: &Habitat, mode: Locomotion, tertiary: bool) -> Vec<Locomotion> {
        let mut locomotions = vec![];
        
        match mode {
            Self::Climbing(_) => match 2.d6() {
                ..=6 => locomotions.push(Self::Slithering),
                7..=11 => locomotions.push(Self::Walking),
                _ => ()
            },
            Self::Digging => match habitat {
                Habitat::Land(_) => match 2.d6() {
                    ..=6 => locomotions.push(Self::Slithering),
                    7..=11 => locomotions.push(Self::Walking),
                    _ => ()
                },
                Habitat::Water(_) => match 2.d6() {
                    ..=5 => {
                        let mode = Self::Slithering;
                        locomotions.push(mode);
                        if !tertiary {
                            locomotions.extend(Self::random_2or3(habitat, mode, true));
                        }
                    },
                    6|7 => {
                        let mode = Self::Walking;
                        locomotions.push(mode);
                        if !tertiary {
                            locomotions.extend(Self::random_2or3(habitat, mode, true));
                        }
                    },
                    8..=11 => locomotions.push(Self::Swimming),
                    _ => ()
                },
                _ => panic!("Habitat {habitat} not defined for random_2or3()!")
            },
            Self::Slithering => match habitat {
                Habitat::Water(_) => match 2.d6() {
                    ..=10 => locomotions.push(Self::Swimming),
                    _ => ()
                },
                _ => ()
            },
            Self::Swimming => match 2.d6() {
                ..=6 => locomotions.push(Self::Slithering),
                7..=9 => locomotions.push(Self::Walking),
                _ => ()
            },
            Self::Walking => match habitat {
                Habitat::Water(_) => match 2.d6() {
                    ..=8 => locomotions.push(Self::Swimming),
                    _ => ()
                },
                _ => ()
            },
            Self::Flight(FlightMode::Winged) => match 2.d6() {
                ..=5 => {
                    let mode = Self::Climbing(false);
                    locomotions.push(mode);
                    if !tertiary {
                        locomotions.extend(Self::random_2or3(habitat, mode, true));
                    }
                }
                6|7 => {
                    let mode = Self::Swimming;
                    locomotions.push(mode);
                    if !tertiary {
                        locomotions.extend(Self::random_2or3(habitat, mode, true));
                    }
                },
                8..=10 => locomotions.push(Self::Walking),
                11 => {
                    let mode = if low!() { Self::Slithering } else { Self::Sliding };
                    locomotions.push(mode);
                    if !tertiary {
                        locomotions.extend(Self::random_2or3(habitat, mode, true));
                    }
                },
                _ => ()
            },
            _ => ()
        }
        
        locomotions
    }

    pub fn is_brachiator(&self) -> bool {
        match self {
            Self::Climbing(true) => true,
            _ => false
        }
    }
}
