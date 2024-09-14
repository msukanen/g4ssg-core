use dicebag::{DiceExt, FixedNumberVariance};
use marginal::MarginalComponent;
use mass::AtmosphericMass;
use rand::Rng;
use toxicity::Toxicity;

use super::{size::Size, terrestrial::{terratype::TerraType, worldtype::WorldType}};

pub mod toxicity;
pub mod marginal;
pub mod mass;

#[derive(Clone)]
pub struct Atmosphere {
    toxicity: Option<Toxicity>,
    suffocating: bool,
    corrosive: bool,
    marginal: Option<MarginalComponent>,
    mass: AtmosphericMass,
    pressure: f64,
}

impl Atmosphere {
    /**
     Generate random potential atmosphere mass (in relation to Earth atmos mass).
     */
    pub fn potential_mass(terratype: &TerraType) -> Option<f64> {
        match terratype {
            TerraType::Tiny(_)                      |
            TerraType::Small(WorldType::Hadean)     |
            TerraType::Small(WorldType::Rock)       |
            TerraType::Medium(WorldType::Chthonian) |
            TerraType::Medium(WorldType::Hadean)    |
            TerraType::Large(WorldType::Chthonian)  => None,
            _ => Some(rand::thread_rng().gen_range(0.5..=1.5).upto_delta(0.05))
        }
    }

    /**
     Generate random atmosphere.
     */
    pub fn random(terratype: &TerraType, gravity: f64) -> Option<Atmosphere> {
        fn pressure(m: f64, f: f64, g: f64) -> f64 { m * f * g }

        let mass = Self::potential_mass(terratype);
        if mass == None { return None }

        let mass = AtmosphericMass::from(mass.unwrap());
        let (toxicity, suffocating, corrosive, marginal) = Self::rng_composition(terratype);
        let pressure = match terratype {
            TerraType::Tiny(_)                     => rand::thread_rng().gen_range(0.000001..=0.001),
            TerraType::Small(WorldType::Rock)      |
            TerraType::Medium(WorldType::Chthonian)|
            TerraType::Large(WorldType::Chthonian) => rand::thread_rng().gen_range(0.001..=0.009999),
            _ => pressure(mass.raw_value(), match terratype {
                TerraType::Small(WorldType::Ice)     => 10.0,
                TerraType::Medium(WorldType::Ammonia)|
                TerraType::Medium(WorldType::Ice)    |
                TerraType::Medium(WorldType::Ocean)  |
                TerraType::Medium(WorldType::Garden) => 1.0,
                TerraType::Medium(WorldType::Greenhouse) => 100.0,
                TerraType::Large(WorldType::Ammonia)|
                TerraType::Large(WorldType::Ice)    |
                TerraType::Large(WorldType::Ocean)  |
                TerraType::Large(WorldType::Garden) => 5.0,
                TerraType::Large(WorldType::Greenhouse) => 500.0,
                _ => panic!("This panic!() should not happen, TerraType::Tiny and a few others ought to be handled by the parent match branch!"),
            }, gravity)
        };

        Some(Atmosphere {
            mass, toxicity, suffocating, corrosive, marginal, pressure,
        })
    }

    /**
     Generate a random gas giant atmosphere.
     */
    pub fn random_gg(size: Size, gravity: f64) -> Atmosphere {
        let _ = size;//TODO: make use of the GG size?
        let mass = AtmosphericMass::Superdense(rand::thread_rng().gen_range(10.0001..=1_000_000.0));
        let pressure = mass.raw_value() * 1_000.0 * gravity;
        Atmosphere {
            mass, pressure,
            toxicity: Some(Toxicity::Lethal),
            suffocating: true,
            corrosive: true,
            marginal: None
        }
    }

    /**
     Generate a random atmosphere composition.
     */
    fn rng_composition(terratype: &TerraType) -> (Option<Toxicity>, bool, bool, Option<MarginalComponent>) {
        match terratype {
            TerraType::Small(WorldType::Ice)  => (Some(if 3.d6() < 16 {Toxicity::Mild} else {Toxicity::High}), true, false, None),
            TerraType::Medium(WorldType::Ice)  => (if 3.d6() > 12 {Some(Toxicity::Mild)} else {None}, true, false, None),
            TerraType::Medium(WorldType::Ocean) => (if 3.d6() > 12 {Some(Toxicity::Mild)} else {None}, true, false, None),
            TerraType::Medium(WorldType::Garden) |
            TerraType::Large(WorldType::Garden)  => (None, false, false, if 3.d6() > 11 {Some(MarginalComponent::random())} else {None}),
            TerraType::Medium(WorldType::Ammonia)   |
            TerraType::Medium(WorldType::Greenhouse)|
            TerraType::Large(WorldType::Greenhouse) |
            TerraType::Large(WorldType::Ammonia)    => (Some(Toxicity::Lethal), true, true, None),
            TerraType::Large(WorldType::Ice)   |
            TerraType::Large(WorldType::Ocean) => (Some(Toxicity::High), true, false, None),
            _ => panic!("Programmer error with rng_composition() detected! {:?} should NOT occur here!", terratype)
        }
    }

    /**
     Check whether the atmosphere is toxic.
     */
    pub fn is_toxic(&self) -> (bool, Option<Toxicity>) {
        if let Some(t) = &self.toxicity {
            (true, Some(t.clone()))
        } else {
            (false, None)
        }
    }

    /**
     Check whether the atmosphere is suffocating.
     */
    pub fn is_suffocating(&self) -> bool {
        self.suffocating
    }

    /**
     Check whether the atmosphere is corrosive.
     */
    pub fn is_corrosive(&self) -> bool {
        self.corrosive
    }

    /**
     Check whether the atmosphere is marginal.
     */
    pub fn is_marginal(&self) -> bool {
        self.marginal == None
    }

    pub fn is_breathable(&self) -> bool {
        let (t, _) = self.is_toxic();
        !t && !self.is_corrosive() && !self.is_suffocating()
    }

    /**
     Get atmospheric mass, if any.
     */
    pub fn mass(&self) -> f64 {
        match self.mass_classification() {
            AtmosphericMass::Trace(x)    |
            AtmosphericMass::VeryThin(x) |
            AtmosphericMass::Thin(x)     |
            AtmosphericMass::Standard(x) |
            AtmosphericMass::Dense(x)    |
            AtmosphericMass::VeryDense(x)|
            AtmosphericMass::Superdense(x)=> *x
        }
    }

    /**
     Get atmospheric mass "classification", if any.
     */
    pub fn mass_classification(&self) -> &AtmosphericMass {
        &self.mass
    }

    /**
     Get atmosphere's Earth-relative pressure.
     */
    pub fn pressure(&self) -> f64 {
        self.pressure
    }
}
