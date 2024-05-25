use self::{primarysense::PrimarySense, vision::Vision};

pub mod primarysense;
pub mod vision;

pub struct Senses {
    primary_sense: PrimarySense,
    vision: Vision,
    hearing: Hearing,
    touch: Touch,
    taste_smell: TasteSmell,
}
