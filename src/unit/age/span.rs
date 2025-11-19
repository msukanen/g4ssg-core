//! Stellar Age Span Stuff

use serde::Deserialize;

/// Stellar age spans.
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum AgeSpan {
    /// hundreds of billions of years to trillions and then some.
    /// Virtually "immortal".
    Infinite,
    /// Just main-sequence span (or 'stable span' in naturally Giant stars' context).
    MSpanOnly(f64),
    /// Main, subgiant and giant spans.
    /// Meaningful only for Common/Intermediate stars.
    MSGSpan(f64, f64, f64)
}

impl Default for AgeSpan {
    fn default() -> Self {
        Self::Infinite
    }
}

impl AgeSpan {
    /// Get "Main-Sequence span" in Gyr.
    pub fn mspan(&self) -> Option<f64> {
        match self {
            Self::Infinite => None,
            Self::MSGSpan(m,..)|
            Self::MSpanOnly(m) => Some(*m)
        }
    }

    /// Get "Subgiant span" in Gyr.
    pub fn sspan(&self) -> Option<f64> {
        match self {
            Self::MSGSpan(_,s,_) => Some(*s),
            _ => None
        }
    }

    /// Get "Giant span" in Gyr.
    pub fn gspan(&self) -> Option<f64> {
        match self {
            Self::MSGSpan(_,_,g) => Some(*g),
            _ => None
        }
    }
}

#[derive(Deserialize)]
struct GiantSpan {
    y: usize,
}

impl From<GiantSpan> for AgeSpan {
    fn from(gs: GiantSpan) -> Self {
        AgeSpan::MSpanOnly(gs.y as f64 / 1_000_000_000.0)
    }
}