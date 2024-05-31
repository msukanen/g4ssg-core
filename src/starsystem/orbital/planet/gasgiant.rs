pub mod arrangement;
pub mod ringsystem;

use std::cmp::max;

use dice::DiceExt;

use crate::starsystem::orbital::{star::limits::orbitlimit::OrbitLimits, OrbitElement, OrbitalInfo};

use self::arrangement::GasGiantArrangement;

use super::{Planet, size::Size};

/**
 Gas Giant, obviously.
 */
#[derive(Clone, Copy)]
pub struct GasGiant {
    arrangement: GasGiantArrangement,
    size: Size,
    major_moons: Vec<Size>,
    moonlets: i32,
    moonlets_outer: i32,
}

impl OrbitalInfo for GasGiant {
    fn distance(&self) -> f64 {
        self.arrangement.distance()
    }
}

impl Planet for GasGiant {
    fn size(&self) -> Size {
        self.size
    }

    fn major_moons(&self) -> &Vec<Size> {
        &self.major_moons
    }

    fn moonlets(&self) -> i32 {
        self.inner_moonlets() + self.outer_moonlets()
    }

}

impl GasGiant {
    pub fn random(inside_snowline_or_first_outside: bool, arrangement: GasGiantArrangement, orbit_limits: &OrbitLimits) -> OrbitElement {
        let size = Size::random_gg(inside_snowline_or_first_outside);

        let moonlets = max(2.d6()
         +  if      arrangement.distance() <= 0.1 {-10}
            else if arrangement.distance()  < 0.5  {-8}
            else if arrangement.distance()  < 0.75 {-6}
            else if arrangement.distance()  < 1.5  {-3}
            else    {0}, 0);

        let moonlets_outer = if arrangement.distance() > 0.5 {
            max(1.d6()
                +   if      arrangement.distance() <= 0.75 {-5}
                    else if arrangement.distance() <= 1.5  {-4}
                    else if arrangement.distance() <= 3.0  {-1}
                    else    {0}, 0)
        } else {0};

        let mut major_moons = vec![];
        if arrangement.distance() > 0.1 {
            let num = 1.d6()
                +   if      arrangement.distance() <= 0.5  {-5}
                    else if arrangement.distance() <= 0.75 {-4}
                    else if arrangement.distance() <= 1.5  {-1}
                    else    {0};
            for _ in 0..num {
                major_moons.push(Size::random_moon(Size::Large))
            }
        }

        OrbitElement::GasGiant(GasGiant {
            arrangement, size,
            moonlets, moonlets_outer, major_moons,
        })
    }

    pub fn inner_moonlets(&self) -> i32 {
        self.moonlets
    }

    pub fn outer_moonlets(&self) -> i32 {
        self.moonlets_outer
    }

    pub fn ring_system(&self) -> 
}
