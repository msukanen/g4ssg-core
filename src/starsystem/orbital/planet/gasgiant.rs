pub mod arrangement;
pub mod ringsystem;

use std::cmp::max;

use dice::DiceExt;
use rand::Rng;
use ringsystem::RingSystem;

use crate::{starsystem::orbital::{star::limits::orbitlimit::OrbitLimits, OrbitElement, OrbitalInfo}, util::distance::mi::Mi};

use self::arrangement::GasGiantArrangement;

use super::{atmosphere::Atmosphere, density::Density, size::Size, Planet};

/**
 Gas Giant, obviously.
 */
#[derive(Clone)]
pub struct GasGiant {
    arrangement: GasGiantArrangement,
    size: Size,
    major_moons: Vec<Size>,
    moonlets: i32,
    moonlets_outer: i32,
    atmosphere: Atmosphere,
    density: Density,
    diameter: f64,
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

    fn atmosphere(&self) -> Option<Atmosphere> {
        Some(self.atmosphere.clone())
    }

    fn gravity(&self) -> f64 {
        
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

        let density = Density::random_gg();

        let diameter = match size {
            Size::Small => Mi::from(rand::thread_rng().gen_range(17_500.0..=25_000.0)),
            Size::Medium => Mi::from(rand::thread_rng().gen_range(47_000.0..=55_000.0)),
            Size::Large => Mi::from(rand::thread_rng().gen_range(79_000.0)),
        };

        OrbitElement::GasGiant(GasGiant {
            arrangement, size,
            moonlets, moonlets_outer, major_moons,
            atmosphere: Atmosphere::random_gg(size),
        })
    }

    pub fn inner_moonlets(&self) -> i32 {
        self.moonlets
    }

    pub fn outer_moonlets(&self) -> i32 {
        self.moonlets_outer
    }

    pub fn ring_system(&self) -> Option<RingSystem> {
        if      self.moonlets() >= 10 {Some(RingSystem::Spectacular)}
        else if self.moonlets() >=  6 {Some(RingSystem::Faint)}
        else    {None}
    }
}
