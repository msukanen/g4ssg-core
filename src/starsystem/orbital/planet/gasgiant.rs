pub mod arrangement;
pub mod ringsystem;

use std::cmp::max;

use dicebag::DiceExt;
use msuk_scifi::unit::{distance::{au::Au, km::Km, mi::Mi, Distance}, mass::{earth::EarthMass, Mass}};
use rand::Rng;
use ringsystem::RingSystem;

use crate::starsystem::orbital::{star::limits::orbitlimit::OrbitLimits, OrbitElement, OrbitalInfo};

use self::arrangement::GasGiantArrangement;

use super::{atmosphere::Atmosphere, density::Density, g, size::Size, Planet};

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
    relative_size: f64,
}

impl OrbitalInfo for GasGiant {
    fn distance(&self) -> Distance {
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

    fn diameter(&self) -> Distance {
        Distance::from(Km::from(Mi::from(7_930.0 * self.relative_size)))
    }

    fn gravity(&self) -> f64 {
        g(&self.density, self.relative_size)
    }

    fn mass(&self) -> Mass {
        Mass::from(EarthMass::from(self.density.value() * self.relative_size * self.relative_size * self.relative_size))
    }
}

impl GasGiant {
    pub fn random(inside_snowline_or_first_outside: bool, arrangement: GasGiantArrangement, orbit_limits: &OrbitLimits) -> OrbitElement {
        let size = Size::random_gg(inside_snowline_or_first_outside);

        let moonlets = max(2.d6()
         +  if      arrangement.distance() <= Distance::Au(Au::from(0.1)) {-10}
            else if arrangement.distance()  < Distance::Au(Au::from(0.5))  {-8}
            else if arrangement.distance()  < Distance::Au(Au::from(0.75)) {-6}
            else if arrangement.distance()  < Distance::Au(Au::from(1.5))  {-3}
            else    {0}, 0);

        let moonlets_outer = if arrangement.distance() > Distance::Au(Au::from(0.5)) {
            max(1.d6()
                +   if      arrangement.distance() <= Distance::Au(Au::from(0.75)) {-5}
                    else if arrangement.distance() <= Distance::Au(Au::from(1.5))  {-4}
                    else if arrangement.distance() <= Distance::Au(Au::from(3.0))  {-1}
                    else    {0}, 0)
        } else {0};

        let mut major_moons = vec![];
        if arrangement.distance() > Distance::Au(Au::from(0.1)) {
            let num = 1.d6()
                +   if      arrangement.distance() <= Distance::Au(Au::from(0.5))  {-5}
                    else if arrangement.distance() <= Distance::Au(Au::from(0.75)) {-4}
                    else if arrangement.distance() <= Distance::Au(Au::from(1.5))  {-1}
                    else    {0};
            for _ in 0..num {
                major_moons.push(Size::random_moon(Size::Large))
            }
        }

        let density = Density::random_gg();

        let relative_size = match size {// rel.values are multiplier to Earth-size
            Size::Small => rand::thread_rng().gen_range(3.4047919294..=4.1614123581),
            Size::Medium => rand::thread_rng().gen_range(5.6746532156..=6.9356872636),
            Size::Large => rand::thread_rng().gen_range(9.079445145..=11.0970996217),
            Size::Tiny => todo!("Tiny GG not implemented...")
        };

        let atmosphere = Atmosphere::random_gg(size, g(&density, relative_size));

        OrbitElement::GasGiant(GasGiant {
            arrangement, size,
            moonlets, moonlets_outer, major_moons,
            atmosphere,
            density, relative_size,
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

impl std::fmt::Display for GasGiant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //TODO: printing in full
        write!(f, "a gas giant")
    }
}
