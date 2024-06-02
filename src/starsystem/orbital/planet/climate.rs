use rand::Rng;

use super::terrestrial::{terratype::TerraType, worldtype::WorldType};

#[derive(Clone, Copy)]
pub enum Climate {
    Frozen(Temperature),
    VeryCold(Temperature),
    Cold(Temperature),
    Chilly(Temperature),
    Cool(Temperature),
    Normal(Temperature),
    Warm(Temperature),
    Tropical(Temperature),
    Hot(Temperature),
    VeryHot(Temperature),
    Infernal(Temperature),
}

impl Climate {
    pub fn avg_temperature(&self) -> f64 {
        match self {
            Self::Frozen(t)  |
            Self::VeryCold(t)|
            Self::Cold(t)    |
            Self::Chilly(t)  |
            Self::Cool(t)    |
            Self::Normal(t)  |
            Self::Warm(t)    |
            Self::Tropical(t)|
            Self::Hot(t)     |
            Self::VeryHot(t) |
            Self::Infernal(t)=> t.avg()
        }
    }
}

impl From<Temperature> for Climate {
    fn from(value: Temperature) -> Self {
        let range = value.avg();
        if      range  < 244.0 {Self::Frozen(value)}
        else if range  < 255.0 {Self::VeryCold(value)}
        else if range  < 266.0 {Self::Cold(value)}
        else if range  < 278.0 {Self::Chilly(value)}
        else if range  < 289.0 {Self::Cool(value)}
        else if range  < 300.0 {Self::Normal(value)}
        else if range  < 311.0 {Self::Warm(value)}
        else if range  < 322.0 {Self::Tropical(value)}
        else if range  < 333.0 {Self::Hot(value)}
        else if range <= 344.0 {Self::VeryHot(value)}
        else                   {Self::Infernal(value)}
    }
}

impl From<&TerraType> for Climate {
    fn from(value: &TerraType) -> Self {
        Climate::from(Temperature::from(value))
    }
}

#[derive(Clone, Copy)]
pub enum Temperature {
    Below(f64),
    Range(f64, f64),
    Above(f64)
}

impl From<&TerraType> for Temperature {
    fn from(terratype: &TerraType) -> Temperature {
        let k = match terratype {
            TerraType::Tiny(WorldType::Ice)     |
            TerraType::Tiny(WorldType::Sulfur)  |
            TerraType::Small(WorldType::Ice)    => rand::thread_rng().gen_range(78.0..=142.0),
            TerraType::Small(WorldType::Hadean) |
            TerraType::Medium(WorldType::Hadean)=> rand::thread_rng().gen_range(49.0..=81.0),
            TerraType::Medium(WorldType::Ammonia)|
            TerraType::Large(WorldType::Ammonia)=> rand::thread_rng().gen_range(137.5..=217.5),
            TerraType::Medium(WorldType::Ice)   |
            TerraType::Large(WorldType::Ice)    => rand::thread_rng().gen_range(70.0..=240.0),
            TerraType::Medium(WorldType::Garden)|
            TerraType::Medium(WorldType::Ocean) |
            TerraType::Large(WorldType::Garden) |
            TerraType::Large(WorldType::Ocean)  => rand::thread_rng().gen_range(247.0..=343.0),
            TerraType::Medium(WorldType::Greenhouse)|
            TerraType::Medium(WorldType::Chthonian) |
            TerraType::Large(WorldType::Greenhouse) |
            TerraType::Large(WorldType::Chthonian)  => rand::thread_rng().gen_range(485.0..=965.0),
            _ => rand::thread_rng().gen_range(128.0..=512.0),
        };

        if k < 244.0 {
            Self::Below(k)
        } else if k > 344.0 {
            Self::Above(k)
        } else {
            if      k < 255.0 {Self::Range(244.0, 254.999)}
            else if k < 266.0 {Self::Range(255.0, 265.999)}
            else if k < 278.0 {Self::Range(266.0, 277.999)}
            else if k < 289.0 {Self::Range(278.0, 288.999)}
            else if k < 300.0 {Self::Range(289.0, 299.999)}
            else if k < 311.0 {Self::Range(300.0, 310.999)}
            else if k < 322.0 {Self::Range(311.0, 321.999)}
            else if k < 333.0 {Self::Range(322.0, 332.999)}
            else              {Self::Range(333.0, 344.0)}
        }
    }
}

impl Temperature {
    pub fn avg(&self) -> f64 {
        match self {
            Self::Below(k) |
            Self::Above(k) => *k,
            Self::Range(a, b) => (a + b) / 2.0
        }
    }
}
