//! Size of beings. The bigger, the better? Not necessarily.

use astrometrics::{AsMass, Cubed, DefoAble, Mass, MetricsInternalType, SpatialUnit};
use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::{life::{chemistry::{ChemicalBasis as CB, ExoticBase}, habitat::{Habitat as H, LandHabitat, WaterHabitat}, locomotion::Locomotion as L, trophics::TrophicLevel as T}, math::ImperialSU};

/// Life form size category (w/o payload).
enum Sz {
    /// sm0l
    S,
    /// human-scale
    H,
    /// larger-than-human scale
    L
} impl Sz {
    #[inline]
    fn random(modf: i32) -> Self {
        match 1.d6() + modf {
            ..=2 => Sz::S,
            3|4  => Sz::H,
            _    => Sz::L
        }
    }
}
/// Life form size.
/// 
/// `hl` stands for "height-or-length", depending on in which
/// dimension the being is at its largest.
/// 
#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, PartialOrd)]
pub enum Size {
    Small { hl: SpatialUnit, wt: Mass },
    HumanScale { hl: SpatialUnit, wt: Mass },
    Large { hl: SpatialUnit, wt: Mass },
} impl Size {
    pub fn random(
        g: Option<MetricsInternalType>,
        cb: CB,
        habitat: H,
        trophics: T,
        locomotion: L,
    ) -> Self {
        let mut modf = 0;
        
        // magnetic things tend to be tiny
        if matches!(cb, CB::Exotic(ExoticBase::Magnetic)) {
            modf -= 4;
        }
        // gravity adds work and thus smaller is better…
        let g = g.unwrap_or_else(|| 0.0);
        modf += match g {
            _ if g <= 0.4  =>  2,
            _ if g <= 0.75 =>  1,
            _ if g >  2.0  => -2,
            _ if g >= 1.5  => -1,
            _ => 0
        };
        // open spaces tend to let things grow…
        modf += match habitat {
            H::Water(w) => 1 +
                match w {
                    WaterHabitat::Banks | WaterHabitat::OpenOceanSurface => 1,
                    WaterHabitat::Lagoon | WaterHabitat::RiverOrStream => -1,
                    _ => 0
                },
            H::Land(l) =>
                match l {
                    LandHabitat::Plains => 1,
                    LandHabitat::IslandOrBeach |
                    LandHabitat::Desert   |
                    LandHabitat::Mountain => -1,
                    _ => 0
                },
            H::Space(_) => 3
        };

        modf += if trophics.contains(T::GRAZING) {1} else {0};
        modf += if trophics.contains(T::PARASITE) {-4} else {0};
        modf += if locomotion.contains(L::SLITHERING) {-1} else {0};
        modf += if locomotion.contains(L::WINGED_FLIGHT) {-3} else {0};
        
        let base_sz = Sz::random(modf);
        let mut sz = match (&base_sz, 1.d6()) {
            (Sz::S, 1) => 0.05.yd2m(),
            (Sz::S, 2) => 0.07.yd2m(),
            (Sz::S, 3) => 0.1.yd2m(),
            (Sz::S, 4) => 0.15.yd2m(),
            (Sz::S, 5) => 0.2.yd2m(),
            (Sz::S, _) => 0.3.yd2m(),
            //
            (Sz::H, 1) => 0.5.yd2m(),
            (Sz::H, 2) => 0.7.yd2m(),
            (Sz::H, 3) => 1.0.yd2m(),
            (Sz::H, 4) => 1.5.yd2m(),
            (Sz::H, 5) => 2.0.yd2m(),
            (Sz::H, _) => 3.0.yd2m(),
            //
            (_,1) => 5.0.yd2m(),
            (_,2) => 7.0.yd2m(),
            (_,3) => 10.0.yd2m(),
            (_,4) => 15.0.yd2m(),
            (_,5) => 20.0.yd2m(),
            _ => ((2.d6() * 10) as f32).yd2m()
        };
        
        let mut wt: Mass;
        if !matches!(habitat, H::Water(_)) && !locomotion.contains(L::BUOYANT_FLIGHT) {
            sz = sz * match g {
                    _ if g >= 5.0 => 0.3,
                    _ if g >= 3.0 => 0.4,
                    _ if g >= 2.25 => 0.5,
                    _ if g >= 1.75 => 0.6,
                    _ if g >= 1.375 => 0.75,
                    _ if g >= 1.125 => 0.9,
                    _ if g >= 0.95 => 1.0,
                    _ if g >= 0.85 => 1.1,
                    _ if g >= 0.75 => 1.2,
                    _ if g >= 0.65 => 1.3,
                    _ if g >= 0.55 => 1.4,
                    _ if g >= 0.45 => 1.6,
                    _ if g >= 0.35 => 1.8,
                    _ if g >= 0.25 => 2.2,
                    _ if g >= 0.15 => 2.9,
                    _ if g >= 0.075 => 4.6,
                    _ => 8.0
                };
            wt = (sz.cubed() * (12.0 * g) as MetricsInternalType).raw().kg();
        } else {
            wt = (sz.cubed() * 12.0 as MetricsInternalType).raw().kg();
        }
        match cb {
            // silicon-based, the heavy weights
            CB::Silicon(_) => wt = wt * 2.0,
            // magnetics are itty-bitty tiny
            CB::Exotic(ExoticBase::Magnetic) => sz = sz / 1_000.0,
            // plasma/hydrogen haven't much of density to speak of, but size…
            CB::Plasma | CB::Hydrogen => { wt = wt / 10.0; sz = sz * 1_000.0 },
            _ => ()
        }
        // space things are light; mass just gets in the way
        if matches!(habitat, H::Space(_)) {
            wt = wt / 5.0;
        }

        match base_sz {
            Sz::S => Self::Small { hl: sz, wt },
            Sz::H => Self::HumanScale { hl: sz, wt },
            Sz::L => Self::Large { hl: sz, wt }
        }
    }
}
