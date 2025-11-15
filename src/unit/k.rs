//! Kelvin'R'Us!
use std::ops::{Div, Mul, Add, Sub};
use paste::paste;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, PartialOrd)]
pub enum Temperature {
    /// White dwarf special case.
    D,
    /// Kelvin.
    K(f64)
}

impl Temperature {
    /// Get Kelvin as `f64`.
    pub fn as_kelvin(&self) -> f64 {
        match self {
            Self::D => 0.0,// special case for White Dorfs…
            Self::K(v) => *v
        }
    }
}

impl From<f64> for Temperature {
    /// Kelvin from `f64`.
    fn from(value: f64) -> Self {
        Self::K(value.max(0.0))// no negatives → clamp to absolute zero…
    }
}

macro_rules! op_temperature_f64 {
    ($trait:ident, $fn:ident) => {
        impl $trait<&Temperature> for f64 {
            type Output = Temperature;
            fn $fn(self, rhs: &Temperature) -> Self::Output {
                match rhs {
                    Temperature::D => Temperature::D,
                    Temperature::K(v) => Temperature::K(self.$fn(v))
                }
            }
        }

        impl $trait<Temperature> for f64 {
            type Output = Temperature;
            fn $fn(self, rhs: Temperature) -> Self::Output {<f64 as $trait<&Temperature>>::$fn(self, &rhs)}
        }
        
        impl $trait<&Temperature> for &Temperature {
            type Output = Temperature;
            fn $fn(self, rhs: &Temperature) -> Self::Output {
                match (self, rhs) {
                    (&Temperature::D, _) |
                    (_, &Temperature::D) => Temperature::D,
                    (&Temperature::K(v1), &Temperature::K(v2)) => Temperature::K(v1.$fn(v2))
                }
            }
        }

        impl $trait<Temperature> for Temperature {
            type Output = Self;
            /// Multiply temperature value by another temperature value. Useful for some math.
            fn $fn(self, rhs: Self) -> Self::Output {<&Temperature as $trait<&Temperature>>::$fn(&self, &rhs)}
        }

        impl $trait<f64> for &Temperature {
            type Output = Temperature;
            fn $fn(self, rhs: f64) -> Self::Output {
                match self {
                    Temperature::D => Temperature::D,
                    Temperature::K(v) => Temperature::K((*v).$fn(rhs))
                }
            }
        }

        impl $trait<f64> for Temperature {
            type Output = Temperature;
            fn $fn(self, rhs: f64) -> Self::Output {<&Temperature as $trait<f64>>::$fn(&self, rhs)}
        }
    };
}
// Build &Temperature and owned Temperature variants...
op_temperature_f64!(Div, div);
op_temperature_f64!(Mul, mul);
op_temperature_f64!(Add, add);
op_temperature_f64!(Sub, sub);

macro_rules! op_temperature_for_integer {
    ($trait:ident, $fn:ident, $bits:expr) => {paste!{
        impl $trait<Temperature> for [<i $bits>] {
            type Output = Temperature;
            fn $fn(self, rhs: Temperature) -> Self::Output {<f64 as $trait<&Temperature>>::$fn(self as f64, &rhs)}
        }
        impl $trait<Temperature> for [<u $bits>] {
            type Output = Temperature;
            fn $fn(self, rhs: Temperature) -> Self::Output {<f64 as $trait<&Temperature>>::$fn(self as f64, &rhs)}
        }
        impl $trait<&Temperature> for [<i $bits>] {
            type Output = Temperature;
            fn $fn(self, rhs: &Temperature) -> Self::Output {<f64 as $trait<&Temperature>>::$fn(self as f64, rhs)}
        }
        impl $trait<&Temperature> for [<u $bits>] {
            type Output = Temperature;
            fn $fn(self, rhs: &Temperature) -> Self::Output {<f64 as $trait<&Temperature>>::$fn(self as f64, rhs)}
        }
        impl $trait<[<i $bits>]> for Temperature {
            type Output = Temperature;
            fn $fn(self, rhs: [<i $bits>]) -> Self::Output {<&Temperature as $trait<f64>>::$fn(&self, rhs as f64)}
        }
        impl $trait<[<i $bits>]> for &Temperature {
            type Output = Temperature;
            fn $fn(self, rhs: [<i $bits>]) -> Self::Output {<&Temperature as $trait<f64>>::$fn(&self, rhs as f64)}
        }
        impl $trait<[<u $bits>]> for Temperature {
            type Output = Temperature;
            fn $fn(self, rhs: [<u $bits>]) -> Self::Output {<&Temperature as $trait<f64>>::$fn(&self, rhs as f64)}
        }
        impl $trait<[<u $bits>]> for &Temperature {
            type Output = Temperature;
            fn $fn(self, rhs: [<u $bits>]) -> Self::Output {<&Temperature as $trait<f64>>::$fn(&self, rhs as f64)}
        }
    }};
}
op_temperature_for_integer!(Div, div, 8);
op_temperature_for_integer!(Div, div, 16);
op_temperature_for_integer!(Div, div, 32);
op_temperature_for_integer!(Div, div, 64);
op_temperature_for_integer!(Div, div, size);

op_temperature_for_integer!(Mul, mul, 8);
op_temperature_for_integer!(Mul, mul, 16);
op_temperature_for_integer!(Mul, mul, 32);
op_temperature_for_integer!(Mul, mul, 64);
op_temperature_for_integer!(Mul, mul, size);

op_temperature_for_integer!(Add, add, 8);
op_temperature_for_integer!(Add, add, 16);
op_temperature_for_integer!(Add, add, 32);
op_temperature_for_integer!(Add, add, 64);
op_temperature_for_integer!(Add, add, size);

op_temperature_for_integer!(Sub, sub, 8);
op_temperature_for_integer!(Sub, sub, 16);
op_temperature_for_integer!(Sub, sub, 32);
op_temperature_for_integer!(Sub, sub, 64);
op_temperature_for_integer!(Sub, sub, size);

pub trait AsTemperature {
    fn k(&self) -> Temperature;
}

impl AsTemperature for f64 {
    fn k(&self) -> Temperature {
        Temperature::K(*self)
    }
}

impl AsTemperature for i32 {
    fn k(&self) -> Temperature { ((*self) as f64).k() }
}

impl Temperature {
    /// Get the underlying raw value as `f64`.
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::D => 0.0,
            Self::K(v) => *v
        }
    }
}

#[cfg(test)]
mod k_tests {
    use super::*;

    #[test]
    fn mulling_around() {
        let x = 1.k();
        let _y = x / 2;
    }
}