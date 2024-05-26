use std::ops::Mul;

use dice::DiceExt;

use super::{base::{ExoticaBase, LifeBase}, habitat::{land::LandHabitat, water::WaterHabitat, Habitat}, locomotion::{FlightMode, Locomotion, LocomotionMode}, trophiclevel::{Herbivore, TrophicLevel, TrophicLevelType}};

#[derive(PartialEq)]
pub enum SizeCategory {
    Small,
    HumanScale,
    Large
}

impl SizeCategory {
    pub fn random(base: &LifeBase, habitat: &Habitat, trophiclevel: &TrophicLevel, locomotion: &Locomotion, local_gravity: f64) -> SizeCategory {
        match 1.d6() + match base {
            LifeBase::Exotica(ExoticaBase::Magnetic) => -4,
            _ => 0
        } + match habitat {
            Habitat::Space => 3,
            Habitat::Water(wh) => 1 + match wh {
                WaterHabitat::OpenOcean |
                WaterHabitat::Banks     => 1,
                WaterHabitat::TropicalLagoon |
                WaterHabitat::RiverOrStream => -1,
                _ => 0
            },
            Habitat::Land(lh) => match lh {
                LandHabitat::Plains => 1,
                LandHabitat::IslandAndBeach |
                LandHabitat::Desert         |
                LandHabitat::Mountain       => -1,
                _ => 0
            },
            Habitat::Exotica => 0
        } + if trophiclevel.is(TrophicLevelType::Herbivore(Herbivore::Grazing)) {1}
            else if trophiclevel.is(TrophicLevelType::Parasite) {-4}
            else {0}
        + if locomotion.is(LocomotionMode::Slithering) {-1} else {0}
        + if locomotion.is(LocomotionMode::Flight(FlightMode::Winged)) {-3} else {0}
        + if      local_gravity <= 0.4  {2}
          else if local_gravity <= 0.75 {1}
          else if local_gravity <= 1.5  {0}
          else if local_gravity <= 2.0 {-1}
          else {-3}
        {
            ..=2 => Self::Small,
            3|4 => Self::HumanScale,
            _ => Self::Large
        }
    }
}

pub struct Size {
    yards: f64,
    lbs: f64,
    size_modifier: i32,
}

impl Mul<f64> for Size {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Size {
            yards: self.yards * rhs,
            lbs: self.lbs * rhs,
            size_modifier: self.size_modifier
        }
    }
}

impl Size {
    pub fn random(size_category: &SizeCategory, base: &LifeBase, habitat: &Habitat, locomotion: &Locomotion, local_gravity: f64) -> Size {
        let no_mod = locomotion.is(LocomotionMode::Flight(FlightMode::Buoyant)) || match habitat {
            Habitat::Water(_) => true,
            _ => false
        };
        let gravity_mod = if no_mod {1.0} else {
            if local_gravity >= 5.0 {0.3}
            else if local_gravity >= 3.5 {0.4}
            else if local_gravity >= 2.5 {0.5}
            else if local_gravity >= 2.0 {0.6}
            else if local_gravity >= 1.5 {0.75}
            else if local_gravity >= 1.25 {0.9}
            else if local_gravity >= 1.0 {1.0}
            else if local_gravity >= 0.9 {1.1}
            else if local_gravity >= 0.8 {1.2}
            else if local_gravity >= 0.7 {1.3}
            else if local_gravity >= 0.6 {1.4}
            else if local_gravity >= 0.5 {1.6}
            else if local_gravity >= 0.4 {1.8}
            else if local_gravity >= 0.3 {2.2}
            else if local_gravity >= 0.2 {2.9}
            else {4.6}};

        let wt_mul = gravity_mod * match base {
            LifeBase::Silicon(_) => 2.0,
            LifeBase::Hydrogen |
            LifeBase::Plasma   => 0.1,
            _ => 1.0
        } * match habitat {
            Habitat::Space => 0.2,
            _ => 1.0
        };
        let sz_mul = gravity_mod * match base {
            LifeBase::Exotica(ExoticaBase::Magnetic) => 0.001,
            _ => 1.0
        };

        let (mut yards, mut size_modifier) = match size_category {
            SizeCategory::Small => match 1.d6() {
                ..=1 => (0.05, -10),
                2 => (0.07, -9),
                3 => (0.1, -8),
                4 => (0.15, -7),
                5 => (0.2, -6),
                _ => (0.3, -5)
            },
            SizeCategory::HumanScale => match 1.d6() {
                ..=1 => (0.5, -4),
                2 => (0.7, -3),
                3 => (1.0, -2),
                4 => (1.5, -1),
                5 => (2.0, 0),
                _ => (3.0, 1)
            },
            SizeCategory::Large => match 1.d6() {
                ..=1 => (5.0, 2),
                2 => (7.0, 3),
                3 => (10.0, 4),
                4 => (15.0, 5),
                5 => (20.0, 6),
                _ => (2.d6() as f64 * 10.0, 10)
            }
        };
        yards *= sz_mul;
        if size_modifier >= 10 {
            size_modifier = 6;
            let step = 10.0;
            let mut yds = 20.0;
            loop {
                if yards < yds + step {
                    break;
                }
                yds += step;
                size_modifier += 1;
            }
        };
        let lbs = (yards / 2.0) * (yards / 2.0) * (yards / 2.0) * 200.0 * wt_mul;
        Size { yards, lbs, size_modifier }
    }
}
