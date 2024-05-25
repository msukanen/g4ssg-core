use dice::{low, DiceExt, HiLo};

use super::{habitat::{land::LandHabitat, water::WaterHabitat, Habitat}, trophiclevel::{Carnivore, Herbivore, TrophicLevel, TrophicLevelType}};

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
pub enum LocomotionMode {
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

pub struct Locomotion {
    modes: Vec<LocomotionMode>
}

impl Locomotion {
    /**
     Generate locomotion type(s), if any.

     Unless result is **None**, the first item in the returned vector is the *primary* locomotion method.
     Secondary/tertiary modes of locomotion may or may not be present.
     */
    pub fn random(habitat: &Habitat, trophiclevel: &TrophicLevel, gasgiant_dweller: bool) -> Locomotion {
        let mut locomotions = vec![];
        let modifier =
            if trophiclevel.is(TrophicLevelType::Carnivore(Carnivore::Chasing)) ||
               trophiclevel.is(TrophicLevelType::Carnivore(Carnivore::Chasing)) ||
               trophiclevel.is(TrophicLevelType::Carnivore(Carnivore::Pouncing))||
               trophiclevel.is(TrophicLevelType::Omnivore)                      ||
               trophiclevel.is(TrophicLevelType::Scavenger)                     ||
               trophiclevel.is(TrophicLevelType::Herbivore(Herbivore::Gathering))
                 {1}
            else {0};
        
        if gasgiant_dweller {
            //
            // Return early for gas giants; habitat doesn't matter there.
            //
            Locomotion {
                modes: vec![match 2.d6() + modifier {
                ..=5 => LocomotionMode::Swimming,
                6..=8 => LocomotionMode::Flight(FlightMode::Winged),
                _ => LocomotionMode::Flight(FlightMode::Buoyant)
            }]};
        }

        match habitat {
            //
            // Land habitats...
            //
            Habitat::Land(h) => match h {
                LandHabitat::Arctic => match 2.d6() + modifier {
                    ..=2 => (),
                    3|4 => locomotions.push(LocomotionMode::Slithering),
                    5|6 => {
                        let mode = LocomotionMode::Swimming;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    7 => {
                        let mode = LocomotionMode::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    8|9 => locomotions.push(LocomotionMode::Walking),
                    10|11 => locomotions.push(LocomotionMode::Flight(FlightMode::Winged)),
                    _ => locomotions.push(LocomotionMode::Special)
                },
                LandHabitat::Desert => match 2.d6() + modifier {
                    ..=2 => (),
                    3|4 => locomotions.push(LocomotionMode::Slithering),
                    5 => {
                        let mode = LocomotionMode::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    6..=8 => locomotions.push(LocomotionMode::Walking),
                    9..=11 => {
                        let mode = LocomotionMode::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(LocomotionMode::Special)
                },
                LandHabitat::IslandAndBeach => match 2.d6() + modifier {
                    ..=2 => (),
                    3|4 => locomotions.push(LocomotionMode::Slithering),
                    5 => {
                        let mode = LocomotionMode::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    6|7 => locomotions.push(LocomotionMode::Walking),
                    8 => {
                        let mode = LocomotionMode::Climbing(true);
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    9 => {
                        let mode = LocomotionMode::Swimming;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    10|11 => {
                        let mode = LocomotionMode::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(LocomotionMode::Special)
                },
                LandHabitat::Mountain => match 2.d6() + modifier {
                    ..=2 => (),
                    3|4 => locomotions.push(LocomotionMode::Slithering),
                    5 => {
                        let mode = LocomotionMode::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    6|7 => {
                        let mode = LocomotionMode::Walking;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    8 => {
                        let mode = LocomotionMode::Climbing(false);
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    9..=11 => {
                        let mode = LocomotionMode::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(LocomotionMode::Special)
                },
                LandHabitat::Plains => match 2.d6() + modifier {
                    ..=2 => (),
                    3|4 => locomotions.push(LocomotionMode::Slithering),
                    5 => {
                        let mode = LocomotionMode::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    6..=8 => locomotions.push(LocomotionMode::Walking),
                    9..=11 => {
                        let mode = LocomotionMode::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(LocomotionMode::Special)
                },
                LandHabitat::Swampland => match 2.d6() + modifier {
                    ..=2 => (),
                    3..=5 => {
                        let mode = LocomotionMode::Swimming;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    6 => locomotions.push(LocomotionMode::Slithering),
                    7 => {
                        let mode = LocomotionMode::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    8 => locomotions.push(LocomotionMode::Walking),
                    9 => {
                        let mode = LocomotionMode::Climbing(1.d2().lo());
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    10|11 => {
                        let mode = LocomotionMode::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(LocomotionMode::Special)
                },
                LandHabitat::Woodlands |
                LandHabitat::Jungle => match 2.d6() + modifier {
                    ..=2 => (),
                    3|4 => locomotions.push(LocomotionMode::Slithering),
                    5 => {
                        let mode = LocomotionMode::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    6|7 => locomotions.push(LocomotionMode::Walking),
                    8|9 => {
                        let mode = LocomotionMode::Climbing(true);
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    10|11 => {
                        let mode = LocomotionMode::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(LocomotionMode::Special)
                }
            },
            //
            // Water habitats...
            //
            Habitat::Water(w) => match w {
                WaterHabitat::Banks |
                WaterHabitat::OpenOcean => match 2.d6() + modifier {
                    ..=3 => (),
                    4 => locomotions.push(LocomotionMode::Floating),
                    5 => locomotions.push(LocomotionMode::Sailing),
                    6..=8 => locomotions.push(LocomotionMode::Swimming),
                    9..=11 => {
                        let mode = LocomotionMode::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(LocomotionMode::Special)
                },
                WaterHabitat::Reef |
                WaterHabitat::DeepOceanVents => match 2.d6() + modifier {
                    ..=5 => (),
                    6 => locomotions.push(LocomotionMode::Floating),
                    7 => {
                        let mode = LocomotionMode::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    8|9 => {
                        let mode = LocomotionMode::Walking;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(LocomotionMode::Swimming)
                },
                WaterHabitat::TropicalLagoon => match 2.d6() + modifier {
                    ..=4 => (),
                    5 => locomotions.push(LocomotionMode::Floating),
                    6 => {
                        let mode = LocomotionMode::Slithering;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    7 => {
                        let mode = LocomotionMode::Walking;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    8 => {
                        let mode = LocomotionMode::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    9 => locomotions.push(LocomotionMode::Swimming),
                    10|11 => locomotions.push(LocomotionMode::Flight(FlightMode::Winged)),
                    _ => locomotions.push(LocomotionMode::Special)
                },
                WaterHabitat::FreshWaterLakes |
                WaterHabitat::SaltWaterSea => match 2.d6() + modifier {
                    ..=3 => (),
                    4 => locomotions.push(LocomotionMode::Floating),
                    5 => {
                        let mode = LocomotionMode::Walking;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    6 => {
                        let mode = LocomotionMode::Slithering;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    7..=9 => locomotions.push(LocomotionMode::Swimming),
                    10|11 => {
                        let mode = LocomotionMode::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(LocomotionMode::Special)
                },
                WaterHabitat::RiverOrStream => match 2.d6() + modifier {
                    ..=3 => (),
                    4 => locomotions.push(LocomotionMode::Floating),
                    5 => {
                        let mode = LocomotionMode::Slithering;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    6 => {
                        let mode = LocomotionMode::Digging;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    7 => {
                        let mode = LocomotionMode::Walking;
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    8|9 => locomotions.push(LocomotionMode::Swimming),
                    10|11 => {
                        let mode = LocomotionMode::Flight(FlightMode::Winged);
                        locomotions.push(mode);
                        locomotions.extend(Locomotion::random_non_primary(habitat, mode))
                    },
                    _ => locomotions.push(LocomotionMode::Special)
                }
            },
            Habitat::Space => match 2.d6() + modifier {
                //TODO: expand on list of Habitat::Space locomotion modes.
                ..=6 => (),
                7..=11 => locomotions.push(LocomotionMode::Flight(FlightMode::SolarSail)),
                12.. => locomotions.push(LocomotionMode::Flight(FlightMode::Rocket))
            },
            //
            // Exotica always uses a "Special" locomotion mode.
            //
            Habitat::Exotica => locomotions.push(LocomotionMode::Special)
        }
        
        Locomotion { modes: locomotions }
    }

    fn random_non_primary(habitat: &Habitat, mode: LocomotionMode) -> Vec<LocomotionMode> {
        Locomotion::random_2or3(habitat, mode, false)
    }

    fn random_2or3(habitat: &Habitat, mode: LocomotionMode, tertiary: bool) -> Vec<LocomotionMode> {
        let mut locomotions = vec![];
        
        match mode {
            LocomotionMode::Climbing(_) => match 2.d6() {
                ..=6 => locomotions.push(LocomotionMode::Slithering),
                7..=11 => locomotions.push(LocomotionMode::Walking),
                _ => ()
            },
            LocomotionMode::Digging => match habitat {
                Habitat::Land(_) => match 2.d6() {
                    ..=6 => locomotions.push(LocomotionMode::Slithering),
                    7..=11 => locomotions.push(LocomotionMode::Walking),
                    _ => ()
                },
                Habitat::Water(_) => match 2.d6() {
                    ..=5 => {
                        let mode = LocomotionMode::Slithering;
                        locomotions.push(mode);
                        if !tertiary {
                            locomotions.extend(Locomotion::random_2or3(habitat, mode, true));
                        }
                    },
                    6|7 => {
                        let mode = LocomotionMode::Walking;
                        locomotions.push(mode);
                        if !tertiary {
                            locomotions.extend(Locomotion::random_2or3(habitat, mode, true));
                        }
                    },
                    8..=11 => locomotions.push(LocomotionMode::Swimming),
                    _ => ()
                },
                _ => panic!("Habitat {habitat} not defined for random_2or3()!")
            },
            LocomotionMode::Slithering => match habitat {
                Habitat::Water(_) => match 2.d6() {
                    ..=10 => locomotions.push(LocomotionMode::Swimming),
                    _ => ()
                },
                _ => ()
            },
            LocomotionMode::Swimming => match 2.d6() {
                ..=6 => locomotions.push(LocomotionMode::Slithering),
                7..=9 => locomotions.push(LocomotionMode::Walking),
                _ => ()
            },
            LocomotionMode::Walking => match habitat {
                Habitat::Water(_) => match 2.d6() {
                    ..=8 => locomotions.push(LocomotionMode::Swimming),
                    _ => ()
                },
                _ => ()
            },
            LocomotionMode::Flight(FlightMode::Winged) => match 2.d6() {
                ..=5 => {
                    let mode = LocomotionMode::Climbing(false);
                    locomotions.push(mode);
                    if !tertiary {
                        locomotions.extend(Locomotion::random_2or3(habitat, mode, true));
                    }
                }
                6|7 => {
                    let mode = LocomotionMode::Swimming;
                    locomotions.push(mode);
                    if !tertiary {
                        locomotions.extend(Locomotion::random_2or3(habitat, mode, true));
                    }
                },
                8..=10 => locomotions.push(LocomotionMode::Walking),
                11 => {
                    let mode = if low!() { LocomotionMode::Slithering } else { LocomotionMode::Sliding };
                    locomotions.push(mode);
                    if !tertiary {
                        locomotions.extend(Locomotion::random_2or3(habitat, mode, true));
                    }
                },
                _ => ()
            },
            _ => ()
        }
        
        locomotions
    }

    /**
     See if locomotion modes include brachiator.
     */
    pub fn is_brachiator(&self) -> bool {
        self.is(LocomotionMode::Climbing(true))
    }

    /**
     See if any locomotion mode matches with...
     */
    pub fn is(&self, locomotion: LocomotionMode) -> bool {
        self.modes.contains(&locomotion)
    }

    /**
     See if primary locomotion mode matches.
     */
    pub fn primary_match(&self, locomotion: LocomotionMode) -> bool {
        if self.modes.is_empty() {
            false
        } else {
            self.modes[0] == locomotion
        }
    }

    /**
     See if secondary locomotion mode matches, if any.
     */
    pub fn secondary_match(&self, locomotion: LocomotionMode) -> bool {
        if self.modes.len() < 2 {
            false
        } else {
            self.modes[1] == locomotion
        }
    }

    /**
     See whether utterly immobile.
     */
    pub fn is_immobile(&self) -> bool {
        self.modes.is_empty()
    }

    /**
     See if we can fly.
     */
    pub fn is_flyer(&self) -> bool {
        for mode in &self.modes {
            match mode {
                LocomotionMode::Flight(_) => return true,
                _ => ()
            }
        }
        false
    }

    /**
     See if we're a climber of any sort.
     */
    pub fn is_climber(&self) -> bool {
        for mode in &self.modes {
            match mode {
                LocomotionMode::Climbing(_) => return true,
                _ => ()
            }
        }
        false
    }
}
