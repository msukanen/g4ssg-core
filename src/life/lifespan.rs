use dice::{DiceExt, HiLo};

use super::{base::{ExoticaBase, LifeBase}, size::Size};

pub enum Lifespan {
    Short(i32),
    Humanlike,
    Long(i32),
    Undying,
}

impl Lifespan {
    pub fn random(base: &LifeBase, size: &Size, sapient: bool) -> Lifespan {
        let mut span = if size.yards() <= 0.4 {
            Self::Short(3)
        } else if size.yards() < 1.5 {
            Self::Short(2)
        } else if size.yards() < 3.0 {
            Self::Short(1)
        } else if size.yards() >= 6.667 {
            Self::Long(1)
        } else {
            Self::Humanlike
        };

        if sapient {
            span = span.shift_up()
        }

        match base {
            LifeBase::Ammonia  |
            LifeBase::Hydrogen => {
                span = span.shift_up();
                if 1.d2().hi() {
                    span = span.shift_up();
                }
            },
            LifeBase::Silicon(_) => {
                span = span.shift_down()
            },
            LifeBase::Exotica(ExoticaBase::Magnetic) => span = Self::Short(1),
            _ => ()
        }

        span
    }

    fn shift_up(&self) -> Lifespan {
        match self {
            Self::Short(n) => if *n == 1 { Self::Humanlike } else { Self::Short(*n-1) },
            Self::Humanlike => Self::Long(1),
            Self::Long(n) => if *n == 3 { Self::Undying } else { Self::Long(*n + 1) },
            Self::Undying => Self::Undying
        }
    }

    fn shift_down(&self) -> Lifespan {
        match self {
            Self::Short(n) => Self::Short(*n-1),
            Self::Humanlike => Self::Short(1),
            Self::Long(n) => if *n == 1 { Self::Humanlike } else { Self::Long(*n - 1) },
            Self::Undying => Self::Long(3)
        }
    }
}
