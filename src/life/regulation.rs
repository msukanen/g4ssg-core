//! Temperature regulation (or lack of such).

use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::life::{breathing::Breathing, habitat::{Habitat, LandHabitat}, size::Size};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum TemperatureRegulation {
    ColdBlooded { survive_frozen: bool },
    Partial,// temperature varies more than with warm-blooded.
    WarmBlooded { metabolism_control: bool },
    Special,
} impl TemperatureRegulation {
    pub(crate) fn random(habitat: Habitat, size: Size, breathing: Option<Breathing>) -> Self {
        let Some(br) = breathing else { return Self::Special };
        let mut modf = match &habitat {
            Habitat::Land(x) => 1 +
                match x {
                    LandHabitat::Arctic    => 2,
                    LandHabitat::Woodlands |
                    LandHabitat::Mountain  => 1,
                    _ => 0
                },

            Habitat::Space(_) => return Self::Special,
            _ => 0
        };
        modf += match br {
            Breathing::Lungs { .. } => 1,
            Breathing::Gills => -1,
            _ => 0
        };
        if matches!(size, Size::HumanScale { .. } | Size::Large { .. }) {
            modf += 1;
        }
        match 2.d6() + modf {
            ..=4 => Self::ColdBlooded { survive_frozen: true },
            5|6  => Self::ColdBlooded { survive_frozen: false },
            7    => Self::Partial,
            8|9  => Self::WarmBlooded { metabolism_control: false },
            _    => Self::WarmBlooded { metabolism_control: true }
        }
    }
}
