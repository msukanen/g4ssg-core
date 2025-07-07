use std::{env, fmt::{Debug, Display}, path::PathBuf, str::FromStr};
use log::{debug, info, warn};
use once_cell::sync::Lazy;

use crate::{core::stellar_age::StellarAge, util::read_lines::read_lines};

const DATA_PATH_VAR: &str = "G4SSG_DATA_PATH";
const DEFAULT_EVO_CSV_FILENAME: &str = "SPECS.CSV";
const CSV_MASSIVE_NUM_PARTS: usize = 5; // Number of CSV data parts for parsing massive stars' specs.
const CSV_AVG_NUM_PARTS: usize = 8;     // Number of CSV data parts for parsing average stars' specs.

/// Data column designators for "average" stars.
enum CSVAvgStarIndex {
    Mass,
    Spectra,
    Temperature,
    LuminosityMin,
    LuminosityMax,
    SpanM,
    SpanS,
    SpanG,
}

/// Data column designators for "massive" stars.
enum CSVMassiveStarIndex {
    Mass,
    Luminosity,
    Temperature,
    SpanM,
}

impl CSVAvgStarIndex {
    /// Convert column designator into an index.
    pub fn as_index(&self) -> usize {
        match self {
            Self::Mass => 0,
            Self::Spectra => 1,
            Self::Temperature => 2,
            Self::LuminosityMin => 3,
            Self::LuminosityMax => 4,
            Self::SpanM => 5,
            Self::SpanS => 6,
            Self::SpanG => 7,
        }
    }
}

impl CSVMassiveStarIndex {
    /// Convert column designator into an index.
    pub fn as_index(&self) -> usize {
        match  self {
            // NOTE: 0th in CSV is ~always~ just 'M' for massives to ease parsing the lines.
            Self::Mass => 1,
            Self::Luminosity => 2,
            Self::Temperature => 3,
            Self::SpanM => 4
        }
    }
}

pub struct StellarEvolution {
    pub massive_star: bool,             // Mass considerably above 2× Sol?
    pub rel_mass: f64,                  // Mass relative to Sol mass.
    pub approx_type: Option<(char, u8)>,// ('A',7), ('G',2), ('M',4), etc.
    pub surface_temperature: u32,       // Surface temperature in K.
    pub luminosity_min: f64,             // Minimum luminance/initial lum for massives, etc.
    pub luminosity_max: Option<f64>,     // Max. luminance before giant stage, etc.
    // Note: M/S/G life spans are in Earth-years.
    pub span_m: Option<f64>,            // Main sequence life span, if applicable.
    pub span_s: Option<f64>,            // Subgiant life span, if applicable.
    pub span_g: Option<f64>,            // Giant life span, if applicable.
}

/// Various giant classifications from the smallest to the largest.
#[derive(PartialEq, PartialOrd)]
pub enum GiantClassification {
    III, II, Ib, Ia, Iap
}

impl Display for GiantClassification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::III => "III",
            Self::II => "II",
            Self::Ib => "Ib",
            Self::Ia => "Ia",
            Self::Iap => "Ia+",
        })
    }
}

/// Various stellar remnant types.
pub enum StellarRemnantClassification {
    WhiteDwarf,
    NeutronStar,
    BlackHole,
}

impl Display for StellarRemnantClassification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Self::WhiteDwarf => "D",
            Self::NeutronStar => "N",
            Self::BlackHole => "X",
        })
    }
}

/// Various stellar evolution stages.
pub enum StellarEvolutionSequence {
    MainSequence,
    Subgiant,
    Giant(GiantClassification),
    Remnant(StellarRemnantClassification),
}

impl Display for StellarEvolutionSequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MainSequence => write!(f, "V"),
            Self::Subgiant => write!(f, "IV"),
            Self::Giant(t) => write!(f, "{}", t),
            Self::Remnant(t) => write!(f, "{}", t),
        }
    }
}

pub static EVOLUTIONS: Lazy<Vec<StellarEvolution>> = Lazy::new(|| {
    info!("Initializing stellar evolution data");
    StellarEvolution::oneshot_evo_csv_parse()
});

impl StellarEvolution {
    /// Parses our CSV data file in one sweep.
    /// 
    /// # Returns
    /// Mass-sorted [StellarEvolution] vector.
    pub fn oneshot_evo_csv_parse() -> Vec<StellarEvolution> {
        let data_dir = env::var(DATA_PATH_VAR).unwrap_or_else(|_| {
            panic!("Required env var '{}' is not set. Please set it before attempting to rerun!", DATA_PATH_VAR);
        });

        let path = PathBuf::from(data_dir).join(DEFAULT_EVO_CSV_FILENAME);
        debug!("Attempting to load data from: {}", path.display());
        let mut evols = vec![];
        
        if let Ok(lines) = read_lines(&path) {
            let mut line_num = 1;
            for line in lines.map_while(Result::ok) {
                if let Some(evo_entry) = Self::parse_csv_line(line_num, &line) {
                    evols.push(evo_entry);
                }
                line_num += 1;
            }
        } else {
            // There was some issue with reading the contents of `path`.
            // We're not going to try be smart and magically fix it - panic
            // and have the user correct their own data manually instead.
            panic!("«Halp!»\nERROR reading from/comprehending contents of {}\n  — OS issue? Network? Data corruption? Fix it!", path.display());
        }

        // sort the evols by mass:
        evols.sort_by(|a, b| {
            // f64 →> partial_cmp()! No NaNnies present in data to begin with.
            a.rel_mass.partial_cmp(&b.rel_mass).unwrap()
        });

        evols
    }

    /// Parse a line of CSV data.
    /// 
    /// # Arguments
    /// * `line_num`— line number (1+) in CSV file.
    /// * `line`— a string, hopefully in CSV format of some sort …
    /// * `evols`— mutable reference to the receiving evolutions 
    pub fn parse_csv_line(line_num: u32, line: &str) -> Option<StellarEvolution> {
        let line = line.trim();
        // Skip all empty and comment lines:
        if line.is_empty() || line.starts_with("#") {
            return None;
        }

        debug!("CSV entry: \"{}\"", line);
        let parts: Vec<&str> = line
                                .split(",")
                                .map(|x| str::trim(x))
                                .collect();
        Some(if line.starts_with("M") {
            Self::parse_massive_star_data(line_num, parts)
        } else {
            Self::parse_avg_star_data(line_num, parts)
        })
    }

    /// Parse a "massive" star's data from the given CVS line parts.
    /// Note that there has to be exactly [CSV_MASSIVE_NUM_PARTS] entries in
    /// the `parts` vector!
    /// 
    /// # Arguments
    /// * `line_num`— CSV line number (1+).
    /// * `parts`— CSV "columns".
    fn parse_massive_star_data(line_num: u32, parts: Vec<&str>) -> StellarEvolution {
        if parts.len() != CSV_MASSIVE_NUM_PARTS {
            panic!("CSV entry failure: expected {CSV_MASSIVE_NUM_PARTS}, found {} - line: {line_num}", parts.len());
        }

        StellarEvolution {
            massive_star: true,
            rel_mass: Self::parse_required(&parts, "Mass", CSVMassiveStarIndex::Mass.as_index()),
            approx_type: None,// massives don't have approx.type.
            surface_temperature: Self::parse_required::<f64>(&parts, "Temperature", CSVMassiveStarIndex::Temperature.as_index()).round() as u32,
            luminosity_min: Self::parse_required(&parts, "Luminosity", CSVMassiveStarIndex::Luminosity.as_index()),
            luminosity_max: None,// massives don't have lum.max.
            span_m: Some(Self::parse_required(&parts, "M-span", CSVMassiveStarIndex::SpanM.as_index())),
            span_s: None,// massives don't go silently — they (super)nova before reaching S/G-span.
            span_g: None
        }
    }

    /// Parse an "average" star's data from the given CVS line parts.
    /// Note that there has to be exactly [CSV_AVG_NUM_PARTS] entries in
    /// the `parts` vector!
    /// 
    /// # Arguments
    /// * `line_num`— CSV line number (1+).
    /// * `parts`— CSV "columns".
    fn parse_avg_star_data(line_num: u32, parts: Vec<&str>) -> StellarEvolution {
        fn from_byr(data: Option<f64>) -> Option<f64> {
            data.map(|val| {
                if val > 1000.0 { val } else { val * 1_000_000_000.0 }
            })
        }
        if parts.len() != CSV_AVG_NUM_PARTS {
            panic!("CSV entry failure: expected {CSV_AVG_NUM_PARTS}, found {} - line: {line_num}", parts.len());
        }

        // Note, ages are converted into ~years~ from Byr, if/when applicable.
        let span_m = from_byr(StellarEvolution::parse_optional(&parts, CSVAvgStarIndex::SpanM.as_index()));
        let span_s = span_m.and_then(|_| from_byr(StellarEvolution::parse_optional(&parts, CSVAvgStarIndex::SpanS.as_index())));
        let span_g = span_s.and_then(|_| from_byr(StellarEvolution::parse_optional(&parts, CSVAvgStarIndex::SpanG.as_index())));

        StellarEvolution {
            massive_star: false,
            rel_mass: StellarEvolution::parse_required(&parts, "Mass", CSVAvgStarIndex::Mass.as_index()),
            approx_type: Some((
                            Self::validate_spectra(parts[CSVAvgStarIndex::Spectra.as_index()]),
                            Self::validate_spectra_intensity(parts[CSVAvgStarIndex::Spectra.as_index()])
                        )),
            surface_temperature: StellarEvolution::parse_required::<f64>(&parts, "Temperature", CSVAvgStarIndex::Temperature.as_index()).round() as u32,
            luminosity_min: StellarEvolution::parse_required(&parts, "L-min", CSVAvgStarIndex::LuminosityMin.as_index()),
            luminosity_max: StellarEvolution::parse_optional(&parts, CSVAvgStarIndex::LuminosityMax.as_index()),
            span_m, span_g, span_s
        }
    }

    /// Parse a field which requires some value.
    /// 
    /// # Arguments
    /// * `parts`— CSV "columns".
    /// * `field_name`— what data the given `index` pertains to. Free-form, given just for error logging purposes.
    /// * `index`— Data column index (0+).
    fn parse_required<T: FromStr + Debug>(
        parts: &[&str],
        field_name: &str,
        index: usize
    ) -> T {
        parts[index].parse::<T>().unwrap_or_else(|_| {
            panic!(
                "{} '{}' [field: {}] is not parsable! Fix!",
                field_name, parts[index], index
            )
        })
    }

    /// Parse a field which does not necessarily need/require a value.
    /// 
    /// Data treated as optional is either:
    /// * an empty column
    /// * a hyphen (or something else hyphen'ish).
    /// 
    /// # Arguments
    /// * `parts`— CSV "columns".
    /// * `index`— Data column index (0+).
    fn parse_optional<T: FromStr>(parts: &[&str], index: usize) -> Option<T> {
        parts.get(index).and_then(|s| {
            let trimmed = s.trim();
            if trimmed.is_empty() || trimmed == "-" || trimmed == "—" {
                None
            } else {
                s.parse().ok()
            }
        })
    }

    /// Validate that the given value corresponds with known spectra types.
    /// 
    /// # Arguments
    /// * `spectra`— A string representing color spectra, hopefully.
    /// 
    /// # Returns
    /// The spectra's color designator ([char]).
    fn validate_spectra(spectra: &str) -> char {
        if let Some(ch) = spectra.chars().nth(0) {
            match ch {
                'O'|'B'|'A'|'F'|'G'|'K'|'M' => ch,
                _ => panic!("Spectra '{ch}' is not one in OBAFGKM!")
            }
        } else { panic!("Spectra not defined! Fix!");}
    }

    /// Validate that the given spectra's intensity value falls into correct range.
    /// The corresponding intensity value is returned.
    /// 
    /// **TODO**: we probably should *panic!* if the intensity indicator is out of bounds (>9).
    /// 
    /// # Arguments
    /// * `spectra`— A string representing color spectra, hopefully.
    /// 
    /// # Returns
    /// Spectra intensity value.
    fn validate_spectra_intensity(spectra: &str) -> u8 {
        if let Some(ch) = spectra.chars().nth(1) {
            if let Some(val) = ch.to_digit(10) { val as u8 }
            else { panic!("Spectra intensity in '{spectra}' is not a digit 0-9! Fix!");}
        } else { panic!("Spectra intensity in '{spectra}' is not a digit 0-9! Fix!");}
    }

    /// Finds the nearest [StellarEvolution] entry by mass using a binary search
    /// on the globally available, sorted [EVOLUTIONS] data.
    ///
    /// # Arguments
    /// * `target_mass` - The f64 mass value to search for.
    ///
    /// # Returns
    /// A static reference to the nearest found entry.
    pub fn find_by_mass(target_mass: f64) -> (usize, &'static StellarEvolution) {
        let search_result = EVOLUTIONS.binary_search_by(|probe| {
            probe.rel_mass.partial_cmp(&target_mass).unwrap_or(std::cmp::Ordering::Less)
        });

        match search_result {
            Ok(index) => (index, &EVOLUTIONS[index]),
            Err(mut index) => {
                if index == 0 {
                    // target is smaller than everything we got; 1st is thus nearest match.
                    (index, EVOLUTIONS.first().unwrap())
                } else if index >= EVOLUTIONS.len() {
                    // target is more massive than we've got data for; last is thus the nearest "match".
                    // TODO: expand the CSV data or extrapolate suitable values
                    //       if target is absurdly larger than our last entry.
                    //       This will need some research...
                    index -= 1;
                    let last_mass = EVOLUTIONS[index].rel_mass;
                    warn!("Data in CSV does not (necessarily) provide an exact match for {target_mass}. Closest \"match\" would be the last entry with {last_mass}.");
                    (index, EVOLUTIONS.last().unwrap())
                } else {
                    // Target is between two elements. Figure out the closest match.
                    let neighbor_before = &EVOLUTIONS[index - 1];
                    let neighbor_after = &EVOLUTIONS[index];
                    let diff_before = (target_mass - neighbor_before.rel_mass).abs();
                    let diff_after = (neighbor_after.rel_mass - target_mass).abs();

                    if diff_before <= diff_after {
                        (index - 1, neighbor_before)
                    } else {
                        (index, neighbor_after)
                    }
                }
            }
        }
    }

    /// Determine [stellar evolution sequence][StellarEvolutionSequence] based on the given
    /// [age][StellarAge].
    /// 
    /// # Arguments
    /// * `age`— stellar age.
    pub fn determine_sequence(&self, age: &StellarAge) -> StellarEvolutionSequence {
        // M-span of None stars are virtually immortal… Let's use 0.0 as span value.
        let Some(span_m) = self.span_m else {
            return StellarEvolutionSequence::MainSequence;
        };
        if span_m <= age.years {
            return StellarEvolutionSequence::MainSequence;
        }
        
        let Some(span_s) = self.span_s else {
            return StellarEvolutionSequence::Remnant(StellarRemnantClassification::WhiteDwarf);
        };
        if span_m + span_s <= age.years {
            return StellarEvolutionSequence::Subgiant;
        }
        
        let Some(span_g) = self.span_g else {
            return StellarEvolutionSequence::Remnant(StellarRemnantClassification::WhiteDwarf);
        };
        if span_m + span_s + span_g <= age.years {
            return StellarEvolutionSequence::Giant(GiantClassification::III);
        }

        StellarEvolutionSequence::Remnant(StellarRemnantClassification::WhiteDwarf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const MOCK_CSV: [&str; 47] = [
    /*  1 */"# Massive stars first (or whenever).",
    /*  2 */"M,100,110000000,50000,9000",
    /*  3 */"M,60,15000000,42000,40000",
    /*  4 */"M,30,920000,40000,330000",
    /*  5 */"M,25,440000,36000,570000",
    /*  6 */" M, 20, 180000, 32000, 1100000",
    /*  7 */"  M ,  15 ,  58000 ,  26000 ,  2600000 ",
    /*  8 */"M,10.0,11000.0,20000.0,9000000.0",
    /*  9 */"M,7.5,3600,18000,20000000",
    /* 10 */"M,5,700,14000,70000000",
    /* 11 */"M,3,90,9800,330000000",
    /* 12 */"# Not so massive stars then (or whenever).",
    /* 13 */"#9,A0,9999,99,99,9,9,9,b,c,d",
    /* 14 */"2.0,A5,8200.0,16.0,20.0,1.3,0.2,0.1",
    /* 15 */" 1.9, A6, 8000.0, 13.0, 16.0, 1.5, 0.2, 0.1",
    /* 16 */"  1.8 ,  A7 ,  7800.0 ,  11.0 ,  13.0 ,  1.8 ,  0.3 ,  0.2 ",
    /* 17 */"1.7,A9,7500.0,8.6,10.0,2.1,0.3,0.2",
    /* 18 */"1.6,F0,7300.0,6.7,8.2,2.5,0.4,0.2",
    /* 19 */"1.5,F2,7000.0,5.1,6.5,3.0,0.5,0.3",
    /* 20 */"1.45,F3,6900.0,4.3,5.7,3.3,0.5,0.3",
    /* 21 */"1.4,F4,6700.0,3.7,5.1,3.7,0.6,0.4",
    /* 22 */"1.35,F5,6600.0,3.1,4.5,4.1,0.6,0.4",
    /* 23 */"1.3,F6,6500.0,2.5,3.9,4.6,0.7,0.4",
    /* 24 */"1.25,F7,6400.0,2.1,3.5,5.2,0.8,0.5",
    /* 25 */"1.2,F8,6300.0,1.7,3.0,5.9,0.9,0.6",
    /* 26 */"1.15,F9,6100.0,1.4,2.6,6.7,1.0,0.6",
    /* 27 */"1.1,G0,6000.0,1.1,2.2,7.7,1.2,0.7",
    /* 28 */"1.05,G1,5900.0,0.87,1.9,8.8,1.4,0.8",
    /* 29 */"1.0,G2,5800.0,0.68,1.6,10.0,1.6,1.0",
    /* 30 */"0.95,G4,5700.0,0.56,1.3,12.0,1.8,1.1",
    /* 31 */"0.9,G6,5500.0,0.45,1.0,14.0,-,-",
    /* 32 */"0.85,G8,5400.0,0.36,0.84,17.0,-,-",
    /* 33 */"0.8,K0,5200.0,0.28,0.65,20.0,-,-",
    /* 34 */"0.75,K2,4900.0,0.23,0.48,24.0,-,-",
    /* 35 */"0.7,K4,4600.0,0.19,0.35,30.0,-,-",
    /* 36 */"0.65,K5,4400.0,0.15,0.25,37.0,-,-",
    /* 37 */"0.6,K6,4200.0,0.13,0.2,42.0,-,-",
    /* 38 */"0.55,K8,4000.0,0.11,0.15,50.0,-,-",
    /* 39 */"0.5,M0,3800.0,0.09,0.11,59.0,-,-",
    /* 40 */"0.45,M1,3600.0,0.07,0.08,70.0,-,-",
    /* 41 */"0.4,M2,3500.0,0.054,-,-,-,-",
    /* 42 */"0.35,M3,3400.0,0.037,-,-,-,-",
    /* 43 */"0.3,M4,3300.0,0.024,-,-,-,-",
    /* 44 */"0.25,M4,3300.0,0.015,-,-,-,-",
    /* 45 */"0.2,M5,3200.0,0.0079,-,-,-,-",
    /* 46 */"0.15,M6,3200.0,0.0036,-,-,-,-",
    /* 47 */"0.1,M7,3100.0,0.0012,-,-,-,-"];
    #[test]
    fn parsing_well_formed_csv() {
        let _ = env_logger::try_init();
        let mut evols: Vec<StellarEvolution> = vec![];
        let mut line_num = 1;
        for line in MOCK_CSV {
            if let Some(evo_entry) = StellarEvolution::parse_csv_line(line_num, &line) {
                evols.push(evo_entry);
            }
            line_num += 1;
        }
    }

    #[test]
    fn oneshot_evo_csv_parse() {
        let _ = env_logger::try_init();
        let evols = StellarEvolution::oneshot_evo_csv_parse();
        assert_eq!(44, evols.len());
    }
}
