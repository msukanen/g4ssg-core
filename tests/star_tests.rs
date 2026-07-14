mod star_tests {
    use crate::{celestial::star::{Star, StarGenCtx, StarLifeStage, UNNAMED}, unit::{Zone, age::StellarPopulation}};

    const AGE_8KYR: StellarPopulation = StellarPopulation::I2(8.0 / 1_000_000.0);
    const AGE_1MYR: StellarPopulation = StellarPopulation::I2(0.001);
    const AGE_5GYR: StellarPopulation = StellarPopulation::I2(5.0);

    #[test]
    fn star_spam() {
        const TIMES: usize = 100_000;
        let mut limits = StarGenCtx::default();
        for _x in 0..TIMES {
            let _star = Star::random(UNNAMED, &AGE_5GYR, &Zone::FREE, &mut limits);
        }
    }

    #[test]
    fn edge_case_and_boundary_values() {
        let _ = env_logger::try_init();
        // to make [0.01,100.0] range simpler to step-by-step at 0.01 interval, use [1,10000] i32 insted and divide…
        for m in 1..=10000 {
            let mut limits = StarGenCtx::default();
            let m = m as f64 / 100.0;
            let star = Star::random(format!("{m:.2}").as_str(), &AGE_8KYR, &Zone::FREE, &mut limits);
            if m < 0.08 {
                assert!(matches!(star.stage, StarLifeStage::B(_)), "Not B?! {:?}", star);
            } else if m < 3.0 {
                assert!(matches!(star.stage, StarLifeStage::M), "Not M?! {:?}", star);
            } else {
                assert!(matches!(star.stage, StarLifeStage::MG | StarLifeStage::SG | StarLifeStage::WR), "Not MG|SG|WR?! {:?}", star);
            }

            let star = Star::random(format!("{m:.2}").as_str(), &AGE_1MYR, &Zone::FREE, &mut limits);
            if m < (25.0 - f64::EPSILON) && matches!(star.stage, StarLifeStage::X) {
                panic!("<25M☉ star ({}) should not result in a black hole!", star.mass)
            }
        }
    }
}
