use super::{breathing::Breathing, habitat::Habitat, size::SizeCategory};

pub enum TemperatureRegulation {
    Special,
    ColdBlooded(bool),
    PartialRegulation,
    WarmBlooded(bool)
}

impl TemperatureRegulation {
    pub fn random(habitat: &Habitat, size_category: &SizeCategory, breathing: &Breathing) -> TemperatureRegulation {
        if habitat.is_space()
        let modifier = match breathing {
            Breathing::Gills => -1,
            Breathing::Lungs => 1,
            _ => 0
        };
    }
}
