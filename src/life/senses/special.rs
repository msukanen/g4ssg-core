//! Special and extra-special senses.

use std::fmt::Display;

use astrometrics::MetricsInternalType;
use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::life::{bodyplan::{BodyPlan, Symmetry}, chemistry::ChemicalBasis, habitat::{Habitat, LandHabitat, WaterHabitat}, locomotion::Locomotion, senses::{Hearing, Vision}, size::Size, trophics::TrophicLevel};

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, Hash)]
    pub struct SpecialSense: u16 {
        const VISION360 = 1 << 0;
        const ABSOLUTE_DIRECTION = 1 << 1;
        const DISCRIMINATORY_HEARING = 1 << 2;
        const PERIPHERAL_VISION = 1 << 3;
        const NIGHT_VISION = 1 << 4;
        /// See high-UV up to and including gamma spectrum. Native to space only.
        /// Creature may have this even if it cannot see normal color spectrum at all.
        const ULTRAVISION = 1 << 5;
        const THERMAL_SIGHT = 1 << 6;
        /// Some fish; some native to space.
        const ELECTRIC_FIELDS_SENSE = 1 << 7;
        const PERFECT_BALANCE = 1 << 8;
        /// Native to space only. Non-acoustic.
        const RADARLIKE_SCAN = 1 << 9;
        /// Hear radio freq. Native to space only.
        const RADIO_HEARING = 1 << 10;
        const DISCRIMINATORY_SMELL = 1 << 11;
        const DISCRIMINATORY_TASTE = 1 << 12;
        const SENSITIVE_TOUCH = 1 << 13;
        const VIBRATION_SENSE = 1 << 14;
    }
} impl Display for SpecialSense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_empty() { return write!(f,"") }

        let mut ss = vec![];
        if self.contains(Self::ABSOLUTE_DIRECTION) {
            ss.push("Absolute Direction");
        }
        if self.contains(Self::DISCRIMINATORY_HEARING) {
            ss.push("Discriminatory Hearing");
        }
        if self.contains(Self::RADIO_HEARING) {
            ss.push("Radio Hearing");
        }
        if self.contains(Self::DISCRIMINATORY_SMELL) {
            ss.push("Discriminatory Smell/Taste");
        }
        if self.contains(Self::DISCRIMINATORY_TASTE) {
            ss.push("Discriminatory Taste");
        }
        if self.contains(Self::ELECTRIC_FIELDS_SENSE) {
            ss.push("Electric Fields Sense");
        }
        if self.contains(Self::VISION360) {
            ss.push("360⁰ Vision")
        }
        if self.contains(Self::NIGHT_VISION) {
            ss.push("Night Vision");
        }
        if self.contains(Self::PERIPHERAL_VISION) {
            ss.push("Peripheral Vision")
        }
        if self.contains(Self::THERMAL_SIGHT) {
            ss.push("Thermal Sight");
        }
        if self.contains(Self::ULTRAVISION) {
            ss.push("Ultravision");
        }
        if self.contains(Self::PERFECT_BALANCE) {
            ss.push("Perfect Balance");
        }
        if self.contains(Self::RADARLIKE_SCAN) {
            ss.push("Radar-like Scan");
        }
        if self.contains(Self::SENSITIVE_TOUCH) {
            ss.push("Sensitive Touch");
        }
        if self.contains(Self::VIBRATION_SENSE) {
            ss.push("Vibration Sense");
        }
        write!(f, "{}", ss.join(", "))
    }
}

impl SpecialSense {
    pub const ALL_SENSES_V: [SpecialSense;15] = [
        Self::VISION360,
        Self::ABSOLUTE_DIRECTION,
        Self::DISCRIMINATORY_HEARING,
        Self::PERIPHERAL_VISION,
        Self::NIGHT_VISION,
        Self::ULTRAVISION,
        Self::THERMAL_SIGHT,
        Self::ELECTRIC_FIELDS_SENSE,
        Self::PERFECT_BALANCE,
        Self::RADARLIKE_SCAN,
        Self::RADIO_HEARING,
        Self::DISCRIMINATORY_SMELL,
        Self::DISCRIMINATORY_TASTE,
        Self::SENSITIVE_TOUCH,
        Self::VIBRATION_SENSE,
    ];

    pub(super) fn random(
        as_count: bool,
        g: Option<MetricsInternalType>,
        cb: ChemicalBasis,
        size: Size,
        habitat: Habitat,
        trophics: TrophicLevel,
        locomotion: Locomotion,
        bodyplan: &BodyPlan,
        vision: Option<Vision>,
        hearing: Option<Hearing>,
    ) -> Self {
        let mut ss = SpecialSense::empty();
        fn _1d6(as_count: bool) -> i32 { if as_count {6} else {1.d6()} }
        fn _2d6(as_count: bool) -> i32 { if as_count {12} else {2.d6()} }

        // Ultravision; can exist as the sole mode of sight.
        let mut ultrav = false;
        match (cb, habitat) {
            (ChemicalBasis::Ammonia, _) => (),
            (_, Habitat::Space(_)) => if _2d6(as_count) >= 11 {
                ultrav = true;
                ss |= SpecialSense::ULTRAVISION;
            },
            _ => ()
        }

        if vision.is_some() || ultrav {
            // 360⁰ vision?
            let mut has360 = false;
            let mut modf = match habitat {
                Habitat::Land(LandHabitat::Plains) |
                Habitat::Land(LandHabitat::Desert) => 1,
                _ => 0
            };
            modf += match bodyplan.symmetry {
                Symmetry::Radial { .. } |
                Symmetry::Spherical { .. } => 1,
                _ => 0
            };
            if trophics.contains(TrophicLevel::HERBIVORE) {
                modf += 1;
            }
            if _2d6(as_count) + modf >= 11 {
                ss |= SpecialSense::VISION360;
                has360 = true;
            }

            // Peripheral vision, if no 360⁰?
            if (!has360 || as_count) && vision.is_some() {
                let mut modf = 0;
                modf += match habitat {
                    Habitat::Land(LandHabitat::Plains) |
                    Habitat::Land(LandHabitat::Desert) => 1,
                    _ => 0
                };
                if trophics.contains(TrophicLevel::HERBIVORE) {
                    modf += 2;
                }
                if _2d6(as_count) + modf >= 10 {
                    ss |= SpecialSense::PERIPHERAL_VISION;
                }
            }

            if vision.is_some() {
                // Night vision?
                let mut modf = match habitat {
                    Habitat::Water(_) => 5,
                    _ => 3
                };
                if trophics.contains(TrophicLevel::CARNIVORE) {
                    modf += 2;
                }
                if _1d6(as_count) + modf >= 11 {
                    ss |= SpecialSense::NIGHT_VISION
                }

                // Thermal
                if !matches!(habitat, Habitat::Water(_)) {
                    let mut modf = if matches!(habitat, Habitat::Land(LandHabitat::Arctic)) {1} else {0};
                    if trophics.contains(TrophicLevel::CARNIVORE) {
                        modf += 1;
                    }
                    if _2d6(as_count) + modf >= 11 {
                        ss |= SpecialSense::THERMAL_SIGHT;
                    }
                }
            }
        }
        
        // Absolute direction…
        let mut modf = if matches!(habitat, Habitat::Water(WaterHabitat::OpenOceanSurface)) {1} else {0};
        if locomotion.intersects(Locomotion::WINGED_FLIGHT|Locomotion::BUOYANT_FLIGHT|Locomotion::DIGGING) {
            modf += 1;
        }
        if _2d6(as_count) + modf >= 11 {
            ss |= SpecialSense::ABSOLUTE_DIRECTION;
        }

        // Discriminatory hearing; req. at least some basic hearing.
        match hearing {
            Some(h) => {
                let modf = match h {
                    Hearing::Acute { sonar: true, .. } => 2,
                    _ => 0
                };
                if _2d6(as_count) + modf >= 11 {
                    ss |= SpecialSense::DISCRIMINATORY_HEARING;
                }
            }
            _ => ()
        }

        // Eletric field sense.
        if matches!(habitat, Habitat::Water(_)|Habitat::Space(_)) {
            let modf = if trophics.contains(TrophicLevel::CARNIVORE) {1} else {0};
            if _2d6(as_count) + modf >= 11 {
                ss |= SpecialSense::ELECTRIC_FIELDS_SENSE;
            }
        }

        // Perfect balance.
        if let Some(g) = g {
            if matches!(habitat, Habitat::Land(_)) {
                let mut modf = if matches!(habitat, Habitat::Land(LandHabitat::Mountain)) {1} else {0};
                if locomotion.contains(Locomotion::CLIMBING) {
                    modf += 2;
                }
                modf += match g {
                    _ if g <= 0.5 => -1,
                    _ if g >= 1.5 => 1,
                    _ => 0
                };
                if _2d6(as_count) + modf >= 11 {
                    ss |= SpecialSense::PERFECT_BALANCE;
                }
            }
        }

        // Radar/lidar/thingydar sense.
        if !matches!(size, Size::Small { .. }) && !matches!(habitat, Habitat::Water(_)) {
            let modf = match habitat {
                Habitat::Space(_) => 2,
                _ => -1
            };
            if _2d6(as_count) + modf >= 12 {
                ss |= SpecialSense::RADARLIKE_SCAN;
            }
        }

        ss
    }
}

#[cfg(test)]
mod specialsense_tests {
    use std::collections::HashSet;
    use astrometrics::MetricsInternalType;
    use crate::life::{bodyplan::{BodyPlan, Limbs, ManipulatorType, Manipulators, Skeleton, Skin, SkinType, Symmetry}, chemistry::ChemicalBasis, habitat::{Habitat, LandHabitat, WaterHabitat}, locomotion::Locomotion, senses::{hearing::Hearing, special::SpecialSense, vision::Vision}, size::Size, trophics::TrophicLevel};
    use super::SpecialSense as S;

    const CASES: usize = 10_000;

    struct Specs {
        g: Option<MetricsInternalType>,
        cb: ChemicalBasis,
        size: Size,
        habitat: Habitat,
        trophics: TrophicLevel,
        locomotion: Locomotion,
        bodyplan: BodyPlan,
        vision: Option<Vision>,
        hearing: Option<Hearing>,
    } impl Default for Specs {
        fn default() -> Self {
            Self {
                g: 1.0.into(),
                cb: ChemicalBasis::Water,
                size: Size::default(),
                habitat: Habitat::Land(LandHabitat::Plains),
                trophics: TrophicLevel::OMNIVORE,
                locomotion: Locomotion::WALKING,
                bodyplan: BodyPlan {
                    symmetry: Symmetry::Bilateral,
                    limbs: Limbs::OneSeg { num: 4, asymmetric: false }.into(),
                    tail: None,
                    manipulators: Manipulators { kind: ManipulatorType::Fine { num: 2 }, prehensile: 0 }.into(),
                    skeleton: Skeleton::Internal.into(),
                    skin: Skin::Skin(SkinType::Normal)
                },
                vision: Vision::Normal.into(),
                hearing: Hearing::Normal.into()
            }
        }
    }

    #[test]
    fn test_full_specialsense_coverage_on_default_spec() {
        _ = env_logger::try_init();
        let mut specs = Specs::default();
        // Land …
        for l in LandHabitat::iter() {
            log::debug!("… LAND ({l}) …");
            specs.habitat = Habitat::Land(l);
            let (pos, was) = was_seen(&specs);
            assert_eq!(pos.bits().count_ones(), was);
        }

        // Water …
        for w in WaterHabitat::iter() {
            log::debug!("… WATER ({w}) …");
            specs.habitat = Habitat::Water(WaterHabitat::OpenOceanSurface);
            let (pos, was) = was_seen(&specs);
            assert_eq!(pos.bits().count_ones(), was);
        }
    }

    fn was_seen(specs: &Specs) -> (SpecialSense, u32) {
        let mut seen = HashSet::new();
        let pos = S::random(true, specs.g, specs.cb, specs.size, specs.habitat, specs.trophics, specs.locomotion, &specs.bodyplan, specs.vision, specs.hearing);
        let mut i = 0;
        loop {
            i += 1;
            let ss = S::random(false, specs.g, specs.cb, specs.size, specs.habitat, specs.trophics, specs.locomotion, &specs.bodyplan, specs.vision, specs.hearing);
            SpecialSense::ALL_SENSES_V.iter().for_each(|a|{
                if ss.contains(*a) && seen.insert(*a) {
                    log::debug!("#{i} {a}");
                }
            });
            if seen.len() as u32 >= pos.bits().count_ones() {
                log::debug!("Found all possible values for spec in {i} checks.");
                break;
            }
        }
        (pos, seen.len() as u32)
    }
}
