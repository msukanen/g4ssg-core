#[macro_export]
macro_rules! approx_eq {
    ($v:expr, $m:expr, $t:expr) => {{
        let v = $v;
        let match_against = $m;
        let threshold = $t;
        println!("v = {}", v);
        println!("m = {}", match_against);
        println!("t = {}", threshold);
        ((match_against - threshold <= v) && (match_against + threshold >= v))
    }};
}
