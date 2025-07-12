pub type DistanceValueType = f64;

/// A common trait between all distance types.
pub trait IsDistance {
    fn value(&self) -> DistanceValueType;
}

/// Define all distance types in one sweep.
macro_rules! define_all_distance_types {
    ( $({$context:ident, $prefix:tt, $symbol:literal}),+ ) => {
        $(
            /// The $context lives here.
            pub struct $context {
                amount: DistanceValueType,
            }

            impl IsDistance for $context {
                fn value(&self) -> DistanceValueType {
                    self.amount
                }
            }

            impl std::fmt::Display for $context {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "{}{}", self.amount, $symbol)
                }
            }
        )+
    };
}
define_all_distance_types!(
    {Km, km, " km"},
    {Au, au, " AU"},
    {Ly, lu, " ly"},
    {Pc, pc, " pc"}
);
