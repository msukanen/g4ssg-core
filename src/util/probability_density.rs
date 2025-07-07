#[cfg(test)]
pub fn find_k(a: f64, b: f64, target_mean: f64) -> f64 {
    fn compute_expected_value(a: f64, b: f64, k: f64) -> f64 {
        let one_minus_k = 1.0 - k;
        if (one_minus_k).abs() < 1e-10 {
            // Avoid division by zero
            return a.ln().abs().recip().mul_add(b.ln(), a.ln()) * 0.5;
        }
        let numerator = (b.powf(2.0 - k) - a.powf(2.0 - k)) / (2.0 - k);
        let denominator = (b.powf(1.0 - k) - a.powf(1.0 - k)) / (1.0 - k);
        numerator / denominator
    }

    let mut low = 0.5;
    let mut high = 10.0;
    for _ in 0..100 {
        let mid = (low + high) / 2.0;
        let mean = compute_expected_value(a, b, mid);
        if (mean - target_mean).abs() < 1e-6 {
            return mid;
        }
        if mean > target_mean {
            low = mid;
        } else {
            high = mid;
        }
    }
    (low + high) / 2.0
}

#[cfg(test)]
pub fn sample_power_law(a: f64, b: f64, k: f64) -> f64 {
    use rand::Rng;
    let mut rng = rand::rng();
    let u: f64 = rng.random();

    // Compute inverse CDF of power-law
    let one_minus_k = 1.0 - k;
    let x = ((b.powf(one_minus_k) - a.powf(one_minus_k)) * u + a.powf(one_minus_k)).powf(1.0 / one_minus_k);
    x
}
