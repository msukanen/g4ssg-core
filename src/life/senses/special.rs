use dice::DiceExt;

use crate::life::{base::LifeBase, habitat::{land::LandHabitat, water::WaterHabitat, ArcticOrDesert, Habitat}, locomotion::{Locomotion, LocomotionMode}, size::SizeCategory, symmetry::Symmetry, trophiclevel::TrophicLevel};

use super::hearing::Hearing;

pub enum Special {
    Vision360,
    AbsoluteDirection,
    DiscriminatoryHearing,
    PeripheralVision,
    NightVision,
    Ultravision,
    DetectHeat,
    DetectElectricFields,
    PerfectBalance,
    ScanningSense
}

impl Special {
    pub fn random(base: &LifeBase, size_category: &SizeCategory, symmetry: &Symmetry, habitat: &Habitat, trophiclevel: &TrophicLevel, locomotion: &Locomotion, hearing: &Hearing, local_gravity: f64) -> Vec<Special> {
        let mut specials = vec![];

        // 360-degree vision
        let mut modifier
         = if habitat.is_desert() ||
              habitat.is(Habitat::Land(LandHabitat::Plains)) {1} else {0}
         + if trophiclevel.is_herbivore(None) {1} else {0}
         + match symmetry {
             Symmetry::Spherical(_) |
             Symmetry::Radial(_)    => 1,
             _ => 0
         };
        if 2.d6() + modifier >= 11 {
            specials.push(Self::Vision360)
        }
        // Abs.Direction
        modifier
         = if habitat.is(Habitat::Water(WaterHabitat::OpenOcean)) {1} else {0}
         + if locomotion.is_flyer() {1} else {0}
         + if locomotion.is(LocomotionMode::Digging) {1} else {0};
        if 2.d6() + modifier >= 11 {
            specials.push(Self::AbsoluteDirection)
        }
        // Discr.Hearing
        if 2.d6() + (if hearing.has_sonar() {2} else {0}) >= 11 {
            specials.push(Self::DiscriminatoryHearing)
        }
        // Perip.Vision
        modifier
         = if trophiclevel.is_herbivore(None) {2} else {0}
         + if habitat.is_desert() || habitat.is(Habitat::Land(LandHabitat::Plains)) {1} else {0};
        if 2.d6() + modifier >= 10 {
            specials.push(Self::PeripheralVision)
        }
        // Night Vision
        modifier
         = if habitat.is_aquatic() {2} else {0}
         + if trophiclevel.is_carnivore(None) {2} else {0};
        if 1.d6() + 3 + modifier >= 11 {
            specials.push(Self::NightVision)
        }
        // Ultravision
        if !base.is_ammonia() && !habitat.is_aquatic() && 2.d6() >= 11 {
            specials.push(Self::Ultravision)
        }
        // Detect Heat
        if !habitat.is_aquatic() {
            modifier
             = if trophiclevel.is_carnivore(None) {1} else {0}
             + if habitat.is_arctic() {1} else {0};
            if 2.d6() + modifier >= 11 {
                specials.push(Self::DetectHeat)
            }
        }
        // Detect Electric Fields
        if habitat.is_aquatic() {
            modifier = if trophiclevel.is_carnivore(None) {1} else {0};
            if 2.d6() + modifier >= 11 {
                specials.push(Self::DetectElectricFields)
            }
        }
        // Perfect Balance
        if !habitat.is_aquatic() && !habitat.is_space() {
            modifier
             = if locomotion.is_climber() {2} else {0}
             + if habitat.is(Habitat::Land(LandHabitat::Mountain)) {1} else {0}
             + if local_gravity <= 0.5 {-1}
               else if local_gravity >= 1.5 {1}
               else {0};
            if 2.d6() + modifier >= 11 {
                specials.push(Self::PerfectBalance)
            }
        }
        // Scanning sense.
        if !habitat.is_aquatic() && match size_category {
            SizeCategory::Small => false,
            _ => true
        } {
            if 2.d6() + if habitat.is_space() {2} else {0} >= 11 {
                specials.push(Self::ScanningSense)
            }
        }

        specials
    }
}
