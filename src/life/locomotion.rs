//! Locomotion, for things that move.

use dicebag::{DiceExt, lo};
use serde::{Deserialize, Serialize};

use crate::life::{habitat::{Habitat as H, LandHabitat as L, SpaceHabitat as S, WaterHabitat as W}, trophics::TrophicLevel as T};

const L_SWIMMING: u32 = 1 << 1;
const L_WALKING: u32 = 1 << 3;
const L_WINGED_FLIGHT: u32 = 1 << 4;
const L_SPECIAL: u32 = 1 << 5;
const L_FLOATING: u32 = 1 << 6;
const L_SOLAR_SAIL: u32 = 1 << 10;
const L_SLIDING: u32 = 1 << 12;
const L_ZERO_INERTIA: u32 = 1 << 13;
const L_NON_LINEAR_VELOCITY: u32 = 1 << 14;
const L_SPACE_GEOMETRY: u32 = 1 << 15;
bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
    // ^-- see the constants above!
    pub struct Locomotion: u32 {
        const SLITHERING = 1 << 0;
        const SWIMMING = L_SWIMMING;
        const DIGGING = 1 << 2;
        const WALKING = L_WALKING;
        const WINGED_FLIGHT = L_WINGED_FLIGHT;
        const SPECIAL = L_SPECIAL;
        const FLOATING = L_FLOATING;
        const SAILING = 1 << 7;
        const BUOYANT_FLIGHT = 1 << 8;
        const CLIMBING = 1 << 9;
        const SOLAR_SAIL = L_SOLAR_SAIL;
        const ROCKET = 1 << 11;
        const SLIDING = L_SLIDING;
        
        // specials
        const ZERO_INERTIA = L_ZERO_INERTIA;
        const PHASE_SHIFT = L_SPECIAL | L_SLIDING | L_ZERO_INERTIA;
        const NON_LINEAR_VELOCITY = L_NON_LINEAR_VELOCITY;
        const QUANTUM_DRIFT = L_SPECIAL | L_FLOATING | L_ZERO_INERTIA | L_NON_LINEAR_VELOCITY;
        const FIELD_DRIFT = L_SPECIAL | L_WALKING;
        const VOID_SLIP = L_SPECIAL | L_SOLAR_SAIL | L_SPACE_GEOMETRY;
        const PLASMA_SKIM = L_SPECIAL | L_SWIMMING;
        const GRAV_HOPPING = L_SPECIAL | L_WINGED_FLIGHT;
    }
}

impl Locomotion {
    pub fn random(habitat: H, trophics: T, gg: bool) -> Self {
        fn r_snd(p: Locomotion, water: bool) -> Locomotion {
            use Locomotion as M;
            let r = 2.d6();
            if p.contains(M::CLIMBING) || (p.contains(M::DIGGING) && !water) {
                match r {
                    ..=6  => M::SLITHERING,
                    ..=11 => M::WALKING,
                    _     => M::empty()
                }
            }
            else if p.contains(M::DIGGING) {
                match r {
                    ..=5  => M::SLITHERING | r_snd(M::SLITHERING, true),
                    ..=11 => M::WALKING,
                    _     => M::empty()
                }
            }
            else if p.contains(M::SLITHERING) {
                match r {
                    ..=10 => M::SWIMMING,
                    _     => M::empty()
                }
            }
            else if p.contains(M::SWIMMING) {
                match r {
                    ..=6 => M::SLITHERING,
                    ..=9 => M::WALKING,
                    _    => M::empty()
                }
            }
            else if p.contains(M::WALKING) {
                match r {
                    ..=8 => M::SWIMMING,
                    _    => M::empty()
                }
            }
            else if p.contains(M::WINGED_FLIGHT) {
                match r {
                    ..=5  => M::CLIMBING,
                    ..=7  => M::SWIMMING,
                    ..=10 => M::WALKING,
                    11    => if lo!() { M::SLIDING } else { M::SLITHERING }
                    _     => M::empty()
                }
            } else {
                M::empty()
            }
        }

        let r = 2.d6() + if trophics.contains(T::CARNIVORE | T::POUNCING) || trophics.contains(T::CARNIVORE | T::CHASING) {1} else {0};
        match habitat {
            H::Land(L::Arctic) => match r {
                ..=2 => Self::empty(),
                ..=4 => Self::SLITHERING,
                ..=6 => Self::SWIMMING | r_snd(Self::SWIMMING, true),
                7    => Self::DIGGING | r_snd(Self::DIGGING, false),
                ..=9 => Self::WALKING,
                ..=11 => Self::WINGED_FLIGHT | r_snd(Self::WINGED_FLIGHT, false),
                _    => Self::SPECIAL
            }
            
            H::Water(W::Banks) |
            H::Water(W::OpenOceanSurface) => match r {
                ..=3 => Self::empty(),
                4    => Self::FLOATING,
                5    => Self::SAILING,
                ..=8 => Self::SWIMMING,
                ..=11 => Self::WINGED_FLIGHT | r_snd(Self::WINGED_FLIGHT, true),
                _    => Self::SPECIAL
            }

            H::Water(W::DeepOceanVents) |
            H::Water(W::Reef) => match r {
                ..=5 => Self::empty(),
                6    => Self::FLOATING,
                7    => Self::DIGGING | r_snd(Self::DIGGING, true),
                8|9  => Self::WALKING | r_snd(Self::WALKING, true),
                _    => Self::SWIMMING
            }

            H::Land(L::Desert) => match r {
                ..=2 => Self::empty(),
                ..=4 => Self::SLITHERING,
                5    => Self::DIGGING | r_snd(Self::DIGGING, false),
                ..=8 => Self::WALKING,
                ..=11 => Self::WINGED_FLIGHT | r_snd(Self::WINGED_FLIGHT, false),
                _    => Self::SPECIAL
            }

            _ if gg => match r {
                ..=5 => Self::SWIMMING,
                ..=8 => Self::WINGED_FLIGHT,
                _    => Self::BUOYANT_FLIGHT
            }

            H::Land(L::IslandOrBeach) => match r {
                ..=2 => Self::empty(),
                ..=4 => Self::SLITHERING,
                5    => Self::DIGGING | r_snd(Self::DIGGING, false),
                6|7  => Self::WALKING,
                8    => Self::CLIMBING | r_snd(Self::CLIMBING, false),
                9    => Self::SWIMMING | r_snd(Self::SWIMMING, false),
                10|11 => Self::WINGED_FLIGHT | r_snd(Self::WINGED_FLIGHT, false),
                _    => Self::SPECIAL
            }

            H::Water(W::Lagoon) => match r {
                ..=4 => Self::empty(),
                5    => Self::FLOATING,
                6    => Self::SLITHERING | r_snd(Self::SLITHERING, true),
                7    => Self::WALKING | r_snd(Self::WALKING, true),
                8    => Self::DIGGING | r_snd(Self::DIGGING, true),
                9    => Self::SWIMMING,
                10|11 => Self::WINGED_FLIGHT,
                _    => Self::SPECIAL
            }

            H::Water(W::Lake) |
            H::Water(W::Sea)  => match r {
                ..=3 => Self::empty(),
                4    => Self::FLOATING,
                5    => Self::WALKING | r_snd(Self::WALKING, true),
                6    => Self::SLITHERING | r_snd(Self::SLITHERING, true),
                ..=9 => Self::SWIMMING,
                10|11 => Self::WINGED_FLIGHT,
                _    => Self::SPECIAL
            }

            H::Land(L::Mountain) => match r {
                ..=2 => Self::empty(),
                3|4  => Self::SLITHERING,
                5    => Self::DIGGING | r_snd(Self::DIGGING, false),
                6|7  => Self::WALKING | r_snd(Self::WALKING, false),
                8    => Self::CLIMBING | r_snd(Self::CLIMBING, false),
                ..=11 => Self::WINGED_FLIGHT | r_snd(Self::WINGED_FLIGHT, false),
                _    => Self::SPECIAL
            }

            H::Land(L::Plains) => match r {
                ..=2 => Self::empty(),
                3|4  => Self::SLITHERING,
                5    => Self::DIGGING | r_snd(Self::DIGGING, false),
                ..=8 => Self::WALKING,
                ..=11 => Self::WINGED_FLIGHT,
                _    => Self::SPECIAL
            }

            H::Land(L::PlanetaryInterior) => match r {
                ..=6 => Self::empty(),
                _    => Self::DIGGING
            }

            H::Water(W::RiverOrStream) => match r {
                ..=3 => Self::empty(),
                4    => Self::FLOATING,
                5    => Self::SLITHERING | r_snd(Self::SLITHERING, true),
                6    => Self::DIGGING | r_snd(Self::DIGGING, true),
                7    => Self::WALKING | r_snd(Self::WALKING, true),
                8|9  => Self::SWIMMING,
                10|11 => Self::WINGED_FLIGHT | r_snd(Self::WINGED_FLIGHT, true),
                _    => Self::SPECIAL
            }

            H::Space(s) if s.intersects(S::DEEP_VOID|S::NEBULAR) => Self::SPECIAL,
            H::Space(_) => match r {
                ..=2 => Self::SPECIAL,
                ..=6 => Self::empty(),
                ..=11 => Self::SOLAR_SAIL,
                _    => Self::ROCKET
            }

            H::Land(L::Swampland) => match r {
                ..=2 => Self::empty(),
                ..=5 => Self::SWIMMING | r_snd(Self::SWIMMING, false),
                6    => Self::SLITHERING,
                7    => Self::DIGGING | r_snd(Self::DIGGING, false),
                8    => Self::WALKING,
                9    => Self::CLIMBING | r_snd(Self::CLIMBING, false),
                10|11 => Self::WINGED_FLIGHT | r_snd(Self::WINGED_FLIGHT, false),
                _    => Self::SPECIAL
            }

            H::Land(L::Woodlands) |
            H::Land(L::Jungle) => match r {
                ..=2 => Self::empty(),
                3|4  => Self::SLITHERING,
                5    => Self::DIGGING | r_snd(Self::DIGGING, false),
                6|7  => Self::WALKING,
                8|9  => Self::CLIMBING | r_snd(Self::CLIMBING, false),
                10|11 => Self::WINGED_FLIGHT | r_snd(Self::WINGED_FLIGHT, false),
                _    => Self::SPECIAL
            }
        }
    }
}