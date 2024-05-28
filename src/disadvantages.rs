use crate::adq::{ControlRated, ADQ};

pub mod racialintolerance;
pub mod xenophobia;
pub mod xenophilia;
pub mod bloodlust;
pub mod orh;
pub mod shortattspan;
pub mod curious;
pub mod incurious;
pub mod selfish;
pub mod oblivious;
pub mod callous;
pub mod loner;
pub mod hidebound;
pub mod cowardice;
pub mod paranoia;
pub mod overconfidence;

pub trait Disadvantage: ADQ + ControlRated {}

pub trait DisadvantageContainer: Sized {
    fn disadvantages(&self) -> &Vec<Box<dyn Disadvantage>>;
}
