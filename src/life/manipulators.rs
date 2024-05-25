use dice::DiceExt;

use super::{habitat::{Habitat, WaterHabitat}, locomotion::{FlightMode, Locomotion}, numlimbs::NumberOfLimbs, trophiclevel::{Herbivore, TrophicLevel}};

pub enum ManipulatorFeature {
    BadGrip,
    NormalDX,
    HighManualDX,
}

pub enum Manipulator {
    LocomotiveOrStriker,
    Manipulator(ManipulatorFeature),
    PrehensileTailOrTrunk,
}

impl Manipulator {
    pub fn random(
        gas_giant: bool,
        sapient: bool,
        num_of_limbs: &NumberOfLimbs,
        habitat: &Habitat,
        trophiclevel: &TrophicLevel,
        locomotion: &Vec<Locomotion>
    ) -> Vec<Manipulator> {
        let modifier =
            if num_of_limbs.count() > 6 {2}
            else if num_of_limbs.count() > 4 {1}
            else if num_of_limbs.count() <= 2 {-1}
            else {0}
        +   match trophiclevel {
                TrophicLevel::Herbivore(Herbivore::Gathering) => 1,
                _ => 0
            }
        +   if locomotion.contains(&Locomotion::Climbing(true)) {2}
            else {0}
        +   if locomotion.contains(&Locomotion::Flight(FlightMode::Winged)) {-1}
            else if locomotion.contains(&Locomotion::Flight(FlightMode::Gliding)) {-1}
            else {0}
        +   if gas_giant {-2}
            else if match habitat {
                Habitat::Water(WaterHabitat::OpenOcean) => true,
                _ => false} {-2}
            else {0};
        let mut manipulators = vec![];
        match if sapient {1.d6()+6} else {2.d6()} + modifier {
            ..=6 => manipulators.push(Self::LocomotiveOrStriker),
            7 => manipulators.push(Self::Manipulator(ManipulatorFeature::BadGrip)),
            8 => manipulators.push(Self::PrehensileTailOrTrunk),
            9 => manipulators.push(Self::Manipulator(ManipulatorFeature::NormalDX)),
            10 => {
                manipulators.push(Self::Manipulator(if 1.d3()==1 {ManipulatorFeature::NormalDX} else {ManipulatorFeature::BadGrip}));
                manipulators.push(Self::Manipulator(if 1.d3()==1 {ManipulatorFeature::NormalDX} else {ManipulatorFeature::BadGrip}))
            },
            11 => for _ in 0..1.d6() {
                manipulators.push(Self::Manipulator(if 1.d3()==1 {ManipulatorFeature::NormalDX} else {ManipulatorFeature::BadGrip}))
            },
            12.. => for _ in 0..1.d6() {
                manipulators.push(Self::Manipulator(if 1.d3()==1 {ManipulatorFeature::HighManualDX} else {ManipulatorFeature::NormalDX}))
            }
        }
        manipulators
    }
}
