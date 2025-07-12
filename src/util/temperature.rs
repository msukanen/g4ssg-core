use std::ops::{Add, Sub, Mul, Div};
use num_traits::Signed;

type TemperatureValueType = f32;

pub struct K {
    amount: TemperatureValueType,
}

macro_rules! impl_base_froms {
    (
        signed:   [$($s_type:ty),+],
        unsigned: [$($u_type:ty),+],
        floats:   [$($f_type:ty),+]
    ) => {
        $(//--- Signed types:
            impl From<$s_type> for K {
                fn from(from:$s_type) -> Self {
                    Self { amount: if from.is_negative() {0.0} else {from as TemperatureValueType}}
                }
            }
        )+

        $(//--- Unsigned types:
            impl From<$u_type> for K {
                fn from(from:$u_type) -> Self {
                    Self { amount: from as TemperatureValueType }
                }
            }
        )+

        $(//--- Floats types:
            // From is the same, essentially, as for signed integers.
            impl From<$f_type> for K {
                fn from(from:$f_type) -> Self {
                    Self { amount: if from.is_negative() {0.0} else {from as TemperatureValueType}}
                }
            }
        )+
    };
}

impl_base_froms!(
    signed:   [i32, i64, isize],
    unsigned: [u32, u64, usize],
    floats:   [f32, f64]
);

macro_rules! impl_unsigned_arith {
    ($( $t:ty ),+) => {
        $(
            impl Add<$t> for K {
                type Output = Self;
                fn add(&self, a:$t) -> Self::Output {
                    Self::Output { amount: self.amount + a as TemperatureValueType}
                }
            }
        )+
    };
}
impl_unsigned_arith!(u32, u64, usize);
