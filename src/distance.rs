pub trait Distance {
    fn parsecs(&self, dx: Self, dy: Self, dz: Self) -> Self;
}

impl Distance for f64 {
    fn parsecs(&self, dx: Self, dy: Self, dz: Self) -> Self {
        (dx*dx + dy*dy + dz*dz).sqrt()
    }
}
