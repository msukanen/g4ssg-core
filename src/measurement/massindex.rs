pub(crate) trait MassIndex {
    fn clamp_mass_index(&self) -> i32;
    fn solar_mass(&self) -> f64;
    fn solar_temperature(&self) -> f64;
    fn solar_lmin(&self) -> f64;
    fn solar_lmax(&self) -> Option<f64>;
    fn solar_agespans(&self) -> (Option<f64>, Option<f64>, Option<f64>);
}

impl MassIndex for i32 {
    fn clamp_mass_index(&self) -> i32 {
        if *self < 0 {0}
        else if *self > 33 {33}
        else {*self}
    }

    /**
     Derive approximate solar mass from mass index.
     */
    fn solar_mass(&self) -> f64 {
        match self {
            ..=0 => 2.0,
            1 => 1.9,
            2 => 1.8,
            3 => 1.7,
            4 => 1.6,
            5 => 1.5,
            6 => 1.45,
            7 => 1.4,
            8 => 1.35,
            9 => 1.3,
            10 => 1.25,
            11 => 1.2,
            12 => 1.15,
            13 => 1.1,
            14 => 1.05,
            15 => 1.0,
            16 => 0.95,
            17 => 0.9,
            18 => 0.85,
            19 => 0.8,
            20 => 0.75,
            21 => 0.7,
            22 => 0.65,
            23 => 0.6,
            24 => 0.55,
            25 => 0.5,
            26 => 0.45,
            27 => 0.4,
            28 => 0.35,
            29 => 0.3,
            30 => 0.25,
            31 => 0.2,
            32 => 0.15,
            33.. => 0.1
        }
    }
    
    /**
     Derive approximate temperature (in Kelvin) from mass index.
     */
    fn solar_temperature(&self) -> f64 {
        match self {
            ..=0 => 8_200.0,
            1 => 8_000.0,
            2 => 7_800.0,
            3 => 7_500.0,
            4 => 7_300.0,
            5 => 7_000.0,
            6 => 6_900.0,
            7 => 6_700.0,
            8 => 6_600.0,
            9 => 6_500.0,
            10 => 6_400.0,
            11 => 6_300.0,
            12 => 6_100.0,
            13 => 6_000.0,
            14 => 5_900.0,
            15 => 5_800.0,
            16 => 5_700.0,
            17 => 5_500.0,
            18 => 5_400.0,
            19 => 5_200.0,
            20 => 4_900.0,
            21 => 4_600.0,
            22 => 4_400.0,
            23 => 4_200.0,
            24 => 4_000.0,
            25 => 3_800.0,
            26 => 3_600.0,
            27 => 3_500.0,
            28 => 3_400.0,
            29|30 => 3_300.0,
            31|32 => 3_200.0,
            33.. => 3_100.0
        }
    }

    /**
     Derive approximate luminosity minimum from mass index.
     */
    fn solar_lmin(&self) -> f64 {
        match self {
            ..=0 => 16.0,
            1 => 13.0,
            2 => 11.0,
            3 => 8.6,
            4 => 6.7,
            5 => 5.1,
            6 => 4.3,
            7 => 3.7,
            8 => 3.1,
            9 => 2.5,
            10 => 2.1,
            11 => 1.7,
            12 => 1.4,
            13 => 1.1,
            14 => 0.87,
            15 => 0.68,
            16 => 0.56,
            17 => 0.45,
            18 => 0.36,
            19 => 0.28,
            20 => 0.23,
            21 => 0.19,
            22 => 0.15,
            23 => 0.13,
            24 => 0.11,
            25 => 0.09,
            26 => 0.07,
            27 => 0.054,
            28 => 0.037,
            29 => 0.024,
            30 => 0.015,
            31 => 0.0079,
            32 => 0.0036,
            33.. => 0.0012
        }
    }

    /**
     Derive approximate luminosity maximum from mass index.
     */
    fn solar_lmax(&self) -> Option<f64> {
        match self {
            ..=0 => Some(20.0),
            1 => Some(16.0),
            2 => Some(13.0),
            3 => Some(10.0),
            4 => Some(8.2),
            5 => Some(6.5),
            6 => Some(5.7),
            7 => Some(5.1),
            8 => Some(4.5),
            9 => Some(3.9),
            10 => Some(3.5),
            11 => Some(3.0),
            12 => Some(2.6),
            13 => Some(2.2),
            14 => Some(1.9),
            15 => Some(1.6),
            16 => Some(1.3),
            17 => Some(1.0),
            18 => Some(0.84),
            19 => Some(0.65),
            20 => Some(0.48),
            21 => Some(0.35),
            22 => Some(0.25),
            23 => Some(0.2),
            24 => Some(0.15),
            25 => Some(0.11),
            26 => Some(0.08),
            27.. => None
        }
    }

    /**
     Derive M-span, S-span and G-span from mass index.
     */
    fn solar_agespans(&self) -> (Option<f64>, Option<f64>, Option<f64>) {
        match self {
            ..=0 => (Some(1.3), Some(0.2), Some(0.1)),
            1 => (Some(1.5), Some(0.2), Some(0.1)),
            2 => (Some(1.8), Some(0.3), Some(0.2)),
            3 => (Some(2.1), Some(0.3), Some(0.2)),
            4 => (Some(2.5), Some(0.4), Some(0.2)),
            5 => (Some(3.0), Some(0.5), Some(0.3)),
            6 => (Some(3.3), Some(0.5), Some(0.3)),
            7 => (Some(3.7), Some(0.6), Some(0.4)),
            8 => (Some(4.1), Some(0.6), Some(0.4)),
            9 => (Some(4.6), Some(0.7), Some(0.4)),
            10 => (Some(5.2), Some(0.8), Some(0.5)),
            11 => (Some(5.9), Some(0.9), Some(0.6)),
            12 => (Some(6.7), Some(1.0), Some(0.6)),
            13 => (Some(7.7), Some(1.2), Some(0.7)),
            14 => (Some(8.8), Some(1.4), Some(0.8)),
            15 => (Some(10.0), Some(1.6), Some(1.0)),
            16 => (Some(12.0), Some(1.8), Some(1.1)),
            17 => (Some(14.0), None, None),
            18 => (Some(17.0), None, None),
            19 => (Some(20.0), None, None),
            20 => (Some(24.0), None, None),
            21 => (Some(30.0), None, None),
            22 => (Some(37.0), None, None),
            23 => (Some(42.0), None, None),
            24 => (Some(50.0), None, None),
            25 => (Some(59.0), None, None),
            26 => (Some(70.0), None, None),
            27.. => (None, None, None),
        }
    }
}
