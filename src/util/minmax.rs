#[macro_export]
macro_rules! maxof {
    ($x:expr, $y:expr) => {{
        let x = $x;
        let y = $y;
        if x > y {x} else {y}
    }};
}
