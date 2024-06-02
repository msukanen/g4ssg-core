use dice::{DiceExt, FixedNumberVariance};
use marginal::MarginalComponent;
use pressure::Pressure;
use rand::Rng;
use toxicity::Toxicity;

use super::{size::Size, terrestrial::{terratype::TerraType, worldtype::WorldType}};

pub mod toxicity;
pub mod marginal;
pub mod pressure;

#[derive(Clone)]
pub struct Atmosphere {
    toxicity: Option<Toxicity>,
    suffocating: bool,
    corrosive: bool,
    marginal: Option<MarginalComponent>,
    pressure: Pressure,
}

impl Atmosphere {
    /**
     Generate random atmosphere.
     */
    pub fn random(terratype: &TerraType) -> Option<Atmosphere> {
        fn gen_mass() -> Option<f64> {
            Some(rand::thread_rng().gen_range(0.5..=1.5).upto_delta(0.05))
        }

        let mass = match terratype {
            TerraType::Tiny(_) => Some(rand::thread_rng().gen_range(0.001..=0.075)),
            TerraType::Small(WorldType::Hadean)     |
            TerraType::Small(WorldType::Rock)       |
            TerraType::Medium(WorldType::Chthonian) |
            TerraType::Medium(WorldType::Hadean)    |
            TerraType::Large(WorldType::Chthonian) => None,
            _ => gen_mass()
        };

        if mass == None { return None }
        
        let mass = mass.unwrap();
        let (toxicity, suffocating, corrosive, marginal) = Self::rng_composition(terratype);
        let pressure = Pressure::new(mass);

        Some(Atmosphere {
            pressure, toxicity, suffocating, corrosive, marginal,
        })
    }

    /**
     Generate a random gas giant atmosphere.
     */
    pub fn random_gg(size: Size) -> Atmosphere {
        let _ = size;//TODO: make use of the GG size?
        Atmosphere {
            pressure: Pressure::Superdense(rand::thread_rng().gen_range(10.0001..=1_000_000.0)),
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
            TerraType::Small(WorldType::Ice) => (Some(if 3.d6() < 16 {Toxicity::Mild} else {Toxicity::High}), true, false, None),
            TerraType::Medium(WorldType::Ice) => (if 3.d6() > 12 {Some(Toxicity::Mild)} else {None}, true, false, None),
            TerraType::Medium(WorldType::Ocean) => (if 3.d6() > 12 {Some(Toxicity::Mild)} else {None}, true, false, None),
            TerraType::Medium(WorldType::Garden) |
            TerraType::Large(WorldType::Garden)  => (None, false, false, if 3.d6() > 11 {Some(MarginalComponent::random())} else {None}),
            TerraType::Medium(WorldType::Ammonia)    |
            TerraType::Medium(WorldType::Greenhouse) |
            TerraType::Large(WorldType::Greenhouse)  |
            TerraType::Large(WorldType::Ammonia)     => (Some(Toxicity::Lethal), true, true, None),
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

    /**
     Get atmospheric mass, if any.
     */
    pub fn mass(&self) -> f64 {
        match self.pressure {
            Pressure::Trace(x)    |
            Pressure::VeryThin(x) |
            Pressure::Thin(x)     |
            Pressure::Standard(x) |
            Pressure::Dense(x)    |
            Pressure::VeryDense(x)|
            Pressure::Superdense(x)=> x
        }
    }
}
