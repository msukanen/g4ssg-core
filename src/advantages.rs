use crate::adq::{ControlRated, ADQ};

pub mod highpainthreshold;
pub mod charitable;
pub mod singleminded;
pub mod selfless;
pub mod empathy;
pub mod gregarious;
pub mod chummy;
pub mod fearlessness;

pub trait Advantage: ADQ + ControlRated {}

pub trait AdvantageContainer: Sized {
    fn advantages(&self) -> &Vec<Box<dyn Advantage>>;
}
