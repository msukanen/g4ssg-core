
pub mod km;
pub mod mi;
pub mod au;
pub mod ly;
pub mod pc;

pub trait Distance {
    fn raw_value(&self) -> f64;
}
