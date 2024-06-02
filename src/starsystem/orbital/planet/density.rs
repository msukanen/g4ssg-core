use super::{core::Core, terrestrial::terratype::TerraType};

#[derive(Clone)]
pub struct Density {
    core: Core
}

impl Density {
    fn random(terratype: &TerraType) -> Density {
        Density { core: Core::random(terratype) }
    }

    pub fn random_gg() -> Density {
        Density { core: Core::random_gg() }
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
