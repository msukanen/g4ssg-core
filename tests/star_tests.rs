mod star_tests {
    use g4ssg_core::{celestial::star::{Star, StarGenCtx, StarLifeStage}, unit::{Zone, age::StellarPopulation}, UNNAMED};

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
}
