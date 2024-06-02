use super::terrestrial::{core::Core, terratype::TerraType};

#[derive(Clone)]
pub struct Density {
    core: Core
}

impl Density {
    fn random(terratype: &TerraType) -> Density {
        Density { core: Core::random(terratype) }
    }

    pub fn value(&self) -> f64 {
        self.core.density()
    }
}

impl From<&TerraType> for Density {
    fn from(value: &TerraType) -> Self {
        Density::random(value)
    }
}

impl From<TerraType> for Density {
    fn from(value: TerraType) -> Self {
        Density::random(&value)
    }
}
