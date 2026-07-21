#[cfg(test)]
mod star_tests {
    use g4ssg_core::{UNNAMED, celestial::{orbital::OrbitContent, star::{Star, StarGenCtx}}, unit::{Zone, age::StellarPopulation}};
    use num_format::{Locale, ToFormattedString};

    const AGE_5GYR: StellarPopulation = StellarPopulation::I2(5.0);
    #[cfg(all(feature = "stresstest", feature = "abandon-all-hope"))]
    const STAR_COUNT: usize = 1_000_000_000;
    #[cfg(all(not(feature = "stresstest"), feature = "abandon-all-hope"))]
    const STAR_COUNT: usize = 10_000_000;
    #[cfg(all(feature = "stresstest", not(feature = "abandon-all-hope")))]
    const STAR_COUNT: usize = 1_000_000;
    #[cfg(all(not(feature = "stresstest"), not(feature = "abandon-all-hope")))]
    const STAR_COUNT: usize = 1_000;

    #[test]
    fn star_spam() {
        use rayon::prelude::*;
        _ = env_logger::try_init();

        // -- RAYON --
        #[cfg(all(not(feature = "stresstest"), not(feature = "abandon-all-hope")))]{
        let now = std::time::Instant::now();
        let stars: Vec<Star> = (0..STAR_COUNT)
            .into_par_iter()
            .with_min_len((STAR_COUNT / 1_000).max(1_000))
            .map(|_| {
                let mut limits = StarGenCtx::default();
                Star::random(UNNAMED, &AGE_5GYR, &Zone::FREE, &mut limits)
            })
            .collect();
        let elapsed_gen = now.elapsed();

        let now = std::time::Instant::now();
        let c_terr: usize = stars
            .into_par_iter()
            .map(|s| s.orbits()
                .par_iter()
                .filter(|(_,oc)| matches!(oc, OrbitContent::T(_)))
                .count()
            ).sum();
        let search_elapsed = now.elapsed();
        log::debug!("Rayon: {} star systems in {elapsed_gen:?}; {} terrestrials in total found in {search_elapsed:?} among them",
            STAR_COUNT.to_formatted_string(&Locale::en),
            c_terr.to_formatted_string(&Locale::en));
        }

        // -- SINGLE-THREADED --
        let now = std::time::Instant::now();
        let stars: Vec<Star> = (0..STAR_COUNT)
            .into_iter()
            .map(|_| {
                let mut limits = StarGenCtx::default();
                Star::random(UNNAMED, &AGE_5GYR, &Zone::FREE, &mut limits)
            })
            .collect();
        let elapsed_gen = now.elapsed();
        let now = std::time::Instant::now();
        let c_terr: usize = stars
            .iter()
            .map(|s| s.orbits().iter().filter(|(_,oc)| matches!(oc, OrbitContent::T(_))).count())
            .sum();
        let search_elapsed = now.elapsed();
        log::debug!("1-thread: {} star systems in {elapsed_gen:?}; {} terrestrials in total found in {search_elapsed:?} among them",
            STAR_COUNT.to_formatted_string(&Locale::en),
            c_terr.to_formatted_string(&Locale::en));
    }
}
