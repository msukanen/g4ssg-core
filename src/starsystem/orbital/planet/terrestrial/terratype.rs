use crate::{starsystem::orbital::planet::size::Size, util::kind::Kind};

use super::worldtype::WorldType;

#[derive(Clone, Debug)]
pub enum TerraType {
    Tiny(WorldType),
    Small(WorldType),
    Medium(WorldType),
    Large(WorldType),
}

impl From<(Size, WorldType)> for TerraType {
    fn from(value: (Size, WorldType)) -> Self {
        match value.0 {
            Size::Tiny => Self::Tiny(value.1),
            Size::Small => Self::Small(value.1),
            Size::Medium => Self::Medium(value.1),
            Size::Large => Self::Large(value.1)
        }
    }
}

impl std::fmt::Display for TerraType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::Tiny(a) => format!("tiny {} world", a.kind()),
            Self::Small(a) => format!("small {} world", a.kind()),
            Self::Medium(a) => format!("medium {} world", a.kind()),
            Self::Large(a) => format!("large {} world", a.kind()),
        })
    }
}
