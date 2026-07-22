//! Trophic level and strategy…

use dicebag::{DiceExt, lo};

use crate::{celestial::terrestrial::climate::Climate, life::habitat::{Habitat, LandHabitat, WaterHabitat}};

bitflags::bitflags! {

    pub struct TrophicLevel: u32 {
        
        const AUTOTROPH = 1 << 0;// usually combined with PHOTO-/CHEMOSYNTHETIC but not necessarily
        const PHOTOSYNTHETIC = 1 << 1;// always combined with AUTOTROPH
        const CHEMOSYNTHETIC = 1 << 2;// always combined with AUTOTROPH
        
        const DECOMPOSER = 1 << 3;
        const SCAVENGER = 1 << 4;

        const OMNIVORE = 1 << 5;
        
        const HERBIVORE = 1 << 6;// always combined with: BROWSING, GRAZING, or GATHERING
        const GRAZING = 1 << 7;// always combined with HERBIVORE
        const GATHERING = 1 << 8;// always combined with HERBIVORE
        const BROWSING = 1 << 9;// always combined with HERBIVORE

        const CARNIVORE = 1 << 10;// always combined with one of the sub-variants below
        const CHASING = 1 << 11;// always combined with CARNIVORE
        const POUNCING = 1 << 12;// always combined with CARNIVORE
        const TRAPPING = 1 << 13;// always combined with CARNIVORE
        const HIJACKING = 1 << 14;// always combined with CARNIVORE
        
        const FILTER_FEEDER = 1 << 15;
        
        const PARASITE = 1 << 16;// mutual excl. SYMBIONT
        const SYMBIONT = 1 << 17;// mutual excl. PARASITE
    }
}

impl TrophicLevel {
    pub fn random(sapient: bool, habitat: Habitat, climate: Climate) -> Self {
        fn r_ordinary(habitat: Habitat, climate: Climate) -> TrophicLevel {
            use TrophicLevel as T;
            match 3.d6() {
                ..=3 => r_ordinary(habitat, climate) | r_ordinary(habitat, climate),
                4 => match 1.d6() {
                        ..=3 if !matches!(habitat, Habitat::Water(WaterHabitat::DeepOceanVents)) => T::AUTOTROPH | T::PHOTOSYNTHETIC,
                        ..=3 => r_ordinary(habitat, climate),
                        ..=5 => T::AUTOTROPH | T::CHEMOSYNTHETIC,
                        _    => T::AUTOTROPH
                    },
                5 => T::DECOMPOSER,
                6 => T::SCAVENGER,
                7 => T::OMNIVORE,
                8|9 => T::HERBIVORE | T::GATHERING,
                10|11 => T::HERBIVORE | if lo!() { T::GRAZING } else { T::BROWSING },
                12 => T::CARNIVORE | T::POUNCING,
                13 => T::CARNIVORE | T::CHASING,
                14 => T::CARNIVORE | T::TRAPPING,
                15 => T::CARNIVORE | T::HIJACKING,
                16 => match habitat {
                        Habitat::Land(LandHabitat::Desert) |
                        Habitat::Land(LandHabitat::Arctic) => T::CARNIVORE | T::TRAPPING,
                        _ => T::FILTER_FEEDER
                    },
                _ => if lo!() { T::PARASITE } else { T::SYMBIONT }
            }
        }

        fn r_sapient(habitat: Habitat, climate: Climate) -> TrophicLevel {
            use TrophicLevel as T;
            match 3.d6() {
                ..=3 => r_sapient(habitat, climate) | r_sapient(habitat, climate),
                4 => if lo!() { T::PARASITE } else { T::SYMBIONT }
                5 => match habitat {
                        Habitat::Land(LandHabitat::Desert) |
                        Habitat::Land(LandHabitat::Arctic) => T::CARNIVORE | T::TRAPPING,
                        _ => T::FILTER_FEEDER
                    },
                6 => T::CARNIVORE | T::POUNCING,
                7 => T::SCAVENGER,
                8|9 => T::HERBIVORE | T::GATHERING,
                10 => T::OMNIVORE,
                11|12 => T::CARNIVORE | T::CHASING,
                13 => T::HERBIVORE | T::GRAZING,
                14 => T::CARNIVORE | T::HIJACKING,
                15|16 => T::CARNIVORE | T::TRAPPING,
                17 => T::DECOMPOSER,
                _ => match 1.d6() {
                    ..=3 => T::AUTOTROPH | T::PHOTOSYNTHETIC,
                    4|5  => T::AUTOTROPH | T::CHEMOSYNTHETIC,
                    _    => T::AUTOTROPH
                }
            }
        }

        if sapient { r_sapient(habitat, climate) } else { r_ordinary(habitat, climate) }
    }
}
