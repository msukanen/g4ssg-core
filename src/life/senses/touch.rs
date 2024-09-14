use dicebag::{DiceExt, HiLo};

use crate::life::{habitat::Habitat, locomotion::{Locomotion, LocomotionMode}, size::SizeCategory, skeleton::Skeleton, trophiclevel::{Carnivore, TrophicLevel}};

use super::vision::Vision;

pub enum AcuteTouch {
    Sensitive,
    VibrationSense
}

pub enum Touch {
    Numb,
    LowDX(i32),
    HumanLevel,
    AcuteTouch(Option<AcuteTouch>)
}

impl Touch {
    pub fn random(
        primary_sense: bool,
        vision: &Vision,
        size_category: &SizeCategory,
        habitat: &Habitat,
        trophiclevel: &TrophicLevel,
        locomotion: &Locomotion,
        skeleton: &Skeleton
    ) -> Touch {
        let modifier
         = if skeleton.is_exoskeleton() {-2} else {0}
         + if habitat.is_aquatic() {2} else {0}
         + if locomotion.is(LocomotionMode::Digging) {2} else {0}
         + if locomotion.is_flyer() {-2} else {0}
         + if vision.is_blind(false) {2} else {0}
         + match size_category {
             SizeCategory::Small => 1,
             _ => 0
         } + if trophiclevel.is_carnivore(Some(Carnivore::Trapping)) {1} else {0}
         + if primary_sense {4} else {0};
        match 2.d6() + modifier {
            ..=2 => Self::Numb,
            3|4 => Self::LowDX(-2),
            5|6 => Self::LowDX(-1),
            7|8 => Self::HumanLevel,
            9|10 => Self::AcuteTouch(None),
            11.. => Self::AcuteTouch(Some(if 1.d2().lo() {AcuteTouch::Sensitive} else {AcuteTouch::VibrationSense}))
        }
    }
}
