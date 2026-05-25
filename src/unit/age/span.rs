//! Stellar Age Span Stuff

use serde::{Deserialize, Deserializer};

/// Stellar age spans.
// NOTE: if messing with the fields here, reflect the changes (if relevant) in the deserializer impl.
#[derive(Debug, Clone)]
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

impl From<usize> for AgeSpan {
    fn from(value: usize) -> Self {<AgeSpan as From<f64>>::from(value as f64)}
}

impl From<f64> for AgeSpan {
    fn from(value: f64) -> Self {
        Self::MSpanOnly(value)
    }
}

impl <'de> Deserialize<'de> for AgeSpan {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum ASHalp {
            Infinite,
            MO(f64),
            MSG(f64,f64,f64),
            G { y: usize }
        }
        
        let ash: ASHalp = ASHalp::deserialize(deserializer)?;
        Ok(match ash {
            ASHalp::G { y } => AgeSpan::MSpanOnly(y as f64 / 1_000_000_000.0),
            ASHalp::MO(v) => AgeSpan::MSpanOnly(v),
            ASHalp::Infinite => AgeSpan::Infinite,
            ASHalp::MSG(m,s,g) => AgeSpan::MSGSpan(m, s, g)
        })
    }
}