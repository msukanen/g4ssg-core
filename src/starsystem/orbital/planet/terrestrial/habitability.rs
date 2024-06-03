use crate::starsystem::orbital::{planet::{atmosphere::{mass::AtmosphericMass, Atmosphere}, climate::Climate, hydrographic::coverage::HydrographicCoverage}, resources::RVM};

pub fn habitability(rvm: RVM, atmosphere: &Option<Atmosphere>, hydrographic: HydrographicCoverage, climate: Climate) -> i32 {
    let mut modifier = match atmosphere {
        None => 0,
        Some(a) => match a.mass_classification() {
            AtmosphericMass::Trace(_) => 0,
            _ => if a.is_breathable() {
                match a.mass_classification() {
                    AtmosphericMass::VeryThin(_) => 1,
                    AtmosphericMass::Thin(_)     => 2,
                    AtmosphericMass::Standard(_) |
                    AtmosphericMass::Dense(_)    => 3,
                    _                            => 1
                }
            } else {
                if a.is_corrosive() { -2 }
                else {
                    match a.is_toxic() {
                        (true, _) => -1,
                        _ => 0
                    }
                }
            }
        }
    };
    
    if let Some(a) = atmosphere {
        if !a.is_marginal() {
            modifier += 1
        }
    }

    modifier += match hydrographic {
        HydrographicCoverage::Barren => 0,
        HydrographicCoverage::Percentage(p) => if p < 60.0 { 1 }
                                                    else if p < 91.0 { 2 }
                                                    else if p < 100.0 { 1 }
                                                    else { 0 }
    };

    if let Some(a) = atmosphere {
        if a.is_breathable() {
            modifier += match climate {
                Climate::VeryHot(_)  |
                Climate::Infernal(_) |
                Climate::Frozen(_)   |
                Climate::VeryCold(_) => 0,
                Climate::Hot(_)      |
                Climate::Cold(_)     => 1,
                Climate::Chilly(_)   |
                Climate::Cool(_)     |
                Climate::Normal(_)   |
                Climate::Warm(_)     |
                Climate::Tropical(_) => 2,
            }
        }
    }

    rvm.modifier() + modifier
}
