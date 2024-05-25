use self::primarysense::PrimarySense;

pub mod primarysense;

pub struct Senses {
    primary_sense: PrimarySense,
    vision: Vision,
    hearing: Hearing,
    touch: Touch,
    taste_smell: TasteSmell,
}
