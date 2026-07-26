//! Life form body plan.

use astrometrics::MetricsInternalType;
use dicebag::DiceExt;
use serde::{Deserialize, Serialize};

use crate::life::{habitat::{Habitat, LandHabitat, WaterHabitat}, locomotion::Locomotion, size::Size, trophics::TrophicLevel};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BodyPlan {
    pub symmetry: Symmetry,
    pub limbs: Option<Limbs>,
    pub tail: Option<Tail>,
    /// if `None` then all existing limbs are just locomotive/striker
    pub manipulators: Option<Manipulators>,
    pub skeleton: Option<Skeleton>,
    pub skin: Skin,
} impl BodyPlan {
    pub fn random(
        gg: bool,
        g: Option<MetricsInternalType>,
        habitat: Habitat,
        trophics: TrophicLevel,
        locomotion: Locomotion,
        size: Size
    ) -> Self {
        let symmetry = Symmetry::random(locomotion, habitat);
        let limbs = Limbs::random(symmetry);
        let tail = Tail::random(locomotion, symmetry);
        let manipulators = Manipulators::random(gg, habitat, trophics, locomotion, symmetry, limbs);
        let skeleton = Skeleton::random(g, size, habitat, locomotion, symmetry);
        let skin = Skin::random(trophics, habitat, locomotion, skeleton);
        Self { symmetry, limbs, tail, manipulators, skeleton, skin }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum Symmetry {
    Bilateral,
    Trilateral,
    Radial { sides: u8 },
    Spherical { sides: u8 },
    Asymmetric,
} impl Symmetry {
    pub(crate) fn random(locomotion: Locomotion, habitat: Habitat) -> Self {
        let mut modf = if locomotion == Locomotion::empty() || locomotion.contains(Locomotion::BUOYANT_FLIGHT) {1} else {0};
        if matches!(habitat, Habitat::Space(_)) {
            modf += 1;
        }
        match 2.d6() + modf {
            ..=7 => Self::Bilateral,
            8    => Self::Trilateral,
            9    => Self::Radial { sides: 1.d6() + 3 },
            10   => Self::Spherical { sides:
                match 1.d6() {
                    1   => 4,
                    2|3 => 6,
                    4   => 8,
                    5   => 12,
                    _   => 20
                }},
            _    => Self::Asymmetric
        }
    }

    #[inline]
    pub const fn sides(&self) -> u8 {
        match self {
            Self::Asymmetric => 1,
            Self::Bilateral => 2,
            Self::Trilateral => 3,
            Self::Radial { sides }    |
            Self::Spherical { sides } => *sides
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq)]
pub enum Limbs {
    OneSeg { num: u8, asymmetric: bool },
    TwoSeg { num: u8 },
    MultiSeg { segments: u8, num: u8 }
} impl Limbs {
    pub(crate) fn random(symmetry: Symmetry) -> Option<Self> {
        fn sym(sides: u8, radial: bool) -> Option<Limbs> {
            let modf: i32 = match (sides, radial) {
                (_, true) => -2,
                (3, _) => -1,
                _ => 0
            };
            match 1.d6() + modf {
                ..=1 => None,
                2    => Limbs::OneSeg { num: sides, asymmetric: false }.into(),
                3    => Limbs::TwoSeg { num: sides * 2 }.into(),
                r    => {
                    let seg = (r - 2).d6() as u8;
                    let num = seg * sides;
                    Limbs::MultiSeg { segments: seg, num }.into()
                }
            }
        }

        match &symmetry {
            Symmetry::Asymmetric => {
                match 2.d6() - 2 {
                    0 => None,
                    num => Self::OneSeg { num, asymmetric: true }.into()
                }
            }

            Symmetry::Spherical { sides } => Self::OneSeg { num: *sides, asymmetric: false }.into(),

            Symmetry::Bilateral => sym(2, false),
            Symmetry::Trilateral => sym(3, false),
            Symmetry::Radial { sides } => sym(*sides, true)
        }
    }

    pub fn count(&self) -> u8 {
        match self {
            // asymmetric things have limbs, or then not — at whim:
            Self::OneSeg { num, asymmetric: true } => 1.d((*num + 1) as usize) - 1,
            Self::OneSeg { num, .. } |
            Self::TwoSeg { num }     => *num,
            Self::MultiSeg { segments, num } => *num * *segments,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub enum Tail {
    Branching { num: u8 },
    Constricting,
    Featureless,
    Gripping,
    Long,
    Striker { barbed: bool },

    DualPurpose { f1: Box<Self>, f2: Box<Self> },
} impl Tail {
    pub(crate) fn random(locomotion: Locomotion, symmetry: Symmetry) -> Option<Self> {
        let modf = match symmetry {
            Symmetry::Spherical { .. } => return None,
            _ => if 1.d3() > 2 {
                if locomotion.contains(Locomotion::SWIMMING) {1} else {0}
            } else { return None }
        };

        fn features(r: i32, modf: i32, symmetry: Symmetry, cap: u8) -> Tail {
            match (r + modf).min(cap as i32) {
                ..=5 => Tail::Featureless,
                6    => Tail::Striker { barbed: false },
                7    => Tail::Long,
                8    => Tail::Constricting,
                9    => Tail::Striker { barbed: true },
                10   => Tail::Gripping,
                11   => Tail::Branching { num: symmetry.sides() },
                _    => {
                    let f1 = features(1.d6() + 5, modf, symmetry, 11);
                    let f2 = features(1.d6() + 5, modf, symmetry, 11);
                    match (&f1, &f2) {
                        (Tail::Striker { .. }, Tail::Striker { .. }) => Tail::Striker { barbed: true },
                        _ => Tail::DualPurpose { f1: Box::new(f1), f2: Box::new(f2) }
                    }
                }
            }
        }

        features(2.d6(), modf, symmetry, 12).into()
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub enum ManipulatorType {
    BadGrip { num: u8 },
    Fine { num: u8 },
    Agile { num: u8 },
    /// Bad-grip one(s) + fine one(s).
    BGF { bg: u8, fine: u8 },
    /// Fine one(s) + agile one(s).
    FA { fine: u8, agile: u8 },
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct Manipulators {
    pub kind: ManipulatorType,
    /// count of prehensiles, if any, mean extra tail/trunk appendage(s) alongside `kind` defined.
    pub prehensile: u8,
} impl Manipulators {
    pub(crate) fn random(
        gg: bool,
        habitat: Habitat,
        trophics: TrophicLevel,
        locomotion: Locomotion,
        symmetry: Symmetry,
        limbs: Option<Limbs>
    ) -> Option<Self> {
        let Some(limbs) = limbs else { return None; };

        let mut modf = 0;
        modf += match limbs.count() {
            ..=2 => -1,
            6.. => 2,
            4.. => 1,
            _ => 0
        };
        if locomotion.contains(Locomotion::WINGED_FLIGHT) {
            modf -= 1;
        }
        if locomotion.contains(Locomotion::CLIMBING) {
            modf += 1;
        }
        if gg || matches!(habitat, Habitat::Water(WaterHabitat::OpenOceanSurface)) {
            modf -= 2;
        }
        if trophics.contains(TrophicLevel::GATHERING) {
            modf += 1;
        }

        fn finger_wigglers(modf: i32, max: u8, symmetry: Symmetry) -> Option<Manipulators> {
            use ManipulatorType as M;
            let mut prehensile = 0;
            let ab = |c: u8| {
                    let mut a = 0;
                    let mut b = 0;
                    for _ in 0..(symmetry.sides() * c).min(max) {
                        if 1.d3() < 3 {
                            a += 1;
                        } else {
                            b += 1;
                        }
                    }
                    (a, b)
                };
            let kind = loop {
                let r = 2.d6() + modf;
                let kind = match r {
                    ..=6 => return None,
                    7 => M::BadGrip { num: symmetry.sides().min(max) },
                    8 => { prehensile += 1; continue; },
                    9 => M::Fine { num: symmetry.sides().min(max) },
                    x if r < 12 => {
                        let c = if x == 10 {2} else {1.d6()};
                        let (a, b) = ab(c);
                        M::BGF { bg: a, fine: b }
                    },
                    _ => {
                        let (a, b) = ab(1.d6());
                        M::FA { fine: a, agile: b }
                    }
                };
                break kind;
            };

            Manipulators { kind, prehensile }.into()
        }

        finger_wigglers(modf, limbs.count(), symmetry)
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum Skeleton {
    Hydrostatic,
    External,
    Internal,
    ///
    CombinationEI,
} impl Skeleton {
    pub(crate) fn random(
        g: Option<MetricsInternalType>,
        size: Size,
        habitat: Habitat,
        locomotion: Locomotion,
        symmetry: Symmetry
    ) -> Option<Self> {
        let mut modf = match size {
            Size::HumanScale { .. } => 1,
            Size::Large { .. } => 2,
            _ => 0
        };
        if matches!(symmetry, Symmetry::Asymmetric) {
            modf -= 1;
        }
        if matches!(habitat, Habitat::Land(_)) {
            modf += 1;
        }
        if locomotion.is_empty() || locomotion.contains(Locomotion::SLITHERING) {
            modf -= 1;
        }
        modf += match g {
            None => -3,
            Some(g) if g < 0.5 => -1,
            Some(g) if g > 1.25 => 1,
            _ => 0
        };
        match 2.d6() + modf {
            ..=3 => None,
            4|5  => Self::Hydrostatic.into(),
            6|7  => Self::External.into(),
            ..=10 => Self::Internal.into(),
            _     => Self::CombinationEI.into()
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq)]
pub enum Skin {
    Exoskeleton (ExoskeletonType),
    Feathers (FeatherType),
    Fur (FurType),
    Skin (SkinType),
    Scales (SkinScaleType),
} impl Default for Skin {
    #[inline]
    fn default() -> Self { Self::Skin(SkinType::default()) }
} impl Skin {
    pub fn random(trophics: TrophicLevel, habitat: Habitat, locomotion: Locomotion, skeleton: Option<Skeleton>) -> Self {
        match skeleton {
            None => Self::Skin(SkinType::Blubber),
            Some(x) if matches!(x, Skeleton::External | Skeleton::CombinationEI) => Self::Exoskeleton(ExoskeletonType::random(habitat, locomotion)),
            _ => match 1.d6() {
                ..=2 => Self::Skin(SkinType::random(trophics, habitat, locomotion)),
                3    => Self::Scales(SkinScaleType::random(trophics, habitat, locomotion)),
                4    => Self::Fur(FurType::random(trophics, habitat, locomotion)),
                5    => Self::Feathers(FeatherType::random(habitat, locomotion)),
                _    => Self::Exoskeleton(ExoskeletonType::random(habitat, locomotion))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SkinType {
    Soft,
    Normal,
    Hide { thick: bool },
    Blubber,
    ArmorShell,
} impl Default for SkinType {
    #[inline(always)] fn default() -> Self { Self::Normal }
} impl SkinType {
    fn random(trophics: TrophicLevel, habitat: Habitat, locomotion: Locomotion) -> Self {
        let mut modf = match &habitat {
            Habitat::Land(LandHabitat::Arctic) => 1,
            Habitat::Land(LandHabitat::Desert) => -1,
            Habitat::Water(_) => 1,
            _ => 0
        };
        if trophics.contains(TrophicLevel::HERBIVORE) {
            modf += 1;
        }
        if locomotion.intersects(Locomotion::WINGED_FLIGHT | Locomotion::BUOYANT_FLIGHT) {
            modf -= 5;
        }
        match 2.d6() + modf {
            ..=4 => Self::Soft,
            5    => Self::Normal,
            6|7  => Self::Hide { thick: false },
            8    => Self::Hide { thick: true },
            9    => Self::ArmorShell,
            _    => Self::Blubber
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum SkinScaleType {
    Normal,
    Tough { grade: u8 },
    ArmorShell,
} impl Default for SkinScaleType {
    #[inline(always)] fn default() -> Self { Self::Normal }
} impl SkinScaleType {
    fn random(trophics: TrophicLevel, habitat: Habitat, locomotion: Locomotion) -> Self {
        let mut modf = if matches!(habitat, Habitat::Land(LandHabitat::Desert)) {1} else {0};
        if trophics.contains(TrophicLevel::HERBIVORE) {
            modf += 1;
        }
        if locomotion.intersects(Locomotion::WINGED_FLIGHT | Locomotion::BUOYANT_FLIGHT) {
            modf -= 2;
        }
        if locomotion.contains(Locomotion::DIGGING) {
            modf -= 1;
        }
        match 2.d6() + modf {
            ..=3 => Self::Normal,
            ..=8 => Self::Tough { grade: 1 },
            ..=10 => Self::Tough { grade: 3 },
            _     => Self::ArmorShell,
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum FurType {
    Soft,
    Coarse { grade: u8 },
    Spines,
} impl Default for FurType {
    #[inline(always)] fn default() -> Self { Self::Coarse { grade: 1 } }
} impl FurType {
    fn random(trophics: TrophicLevel, habitat: Habitat, locomotion: Locomotion) -> Self {
        let mut modf = match &habitat {
            Habitat::Land(LandHabitat::Arctic) => 1,
            Habitat::Land(LandHabitat::Desert) => -1,
            _ => 0
        };
        if trophics.contains(TrophicLevel::HERBIVORE) {
            modf += 1;
        }
        if locomotion.intersects(Locomotion::WINGED_FLIGHT | Locomotion::BUOYANT_FLIGHT) {
            modf -= 1;
        }
        match 2.d6() + modf {
            ..=5 => Self::Soft,
            6|7  => Self::Coarse { grade: 1 },
            8|9  => Self::Coarse { grade: 2 },
            10|11 => Self::Coarse { grade: 3 },
            _     => Self::Spines
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum FeatherType {
    Soft,
    Warm,
    Thick { grade: u8 },
    Spines,
} impl Default for FeatherType {
    #[inline(always)] fn default() -> Self { Self::Warm }
} impl FeatherType {
    fn random(habitat: Habitat, locomotion: Locomotion) -> Self {
        let mut modf = if locomotion.intersects(Locomotion::WINGED_FLIGHT | Locomotion::BUOYANT_FLIGHT) {1} else {0};
        modf += match &habitat {
            Habitat::Land(LandHabitat::Arctic) => 1,
            Habitat::Land(LandHabitat::Desert) => -1,
            _ => 0
        };
        match 2.d6() + modf {
            ..=5 => Self::Soft,
            ..=8 => Self::Warm,
            9|10 => Self::Thick { grade: 1 },
            11   => Self::Thick { grade: 2 },
            _    => Self::Spines
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ExoskeletonType {
    Light,
    Tough,
    Heavy,
    ArmorShell,
} impl Default for ExoskeletonType {
    #[inline(always)] fn default() -> Self { Self::Tough }
} impl ExoskeletonType {
    fn random(habitat: Habitat, locomotion: Locomotion) -> Self {
        let mut modf = if matches!(habitat, Habitat::Water(_)) {1} else {0};
        if locomotion.is_empty() {
            modf += 1;
        } else if locomotion.intersects(Locomotion::WINGED_FLIGHT | Locomotion::BUOYANT_FLIGHT) {
            modf -= 2;
        }
        match 1.d6() + modf {
            ..=2 => Self::Light,
            3|4  => Self::Tough,
            5    => Self::Heavy,
            _    => Self::ArmorShell
        }
    }
}
