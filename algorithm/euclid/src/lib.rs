//! # Euclid
//! 
//! 
/// function
pub fn euclid(mut x: u64, mut y: u64) -> u64 {
    while y > 0 {
        (x, y) = (y, x % y);
    }
    return x
}

/// function but bug
pub fn ext_euclid(x: i64, y: i64) -> (i64, i64) {
    let (a, g) = {
        let mut x = x;
        let mut y = y;
        let mut u = 0;
        let mut v = 0;
        while x != 0 {
            let q = y / x;
            y -= q * x;
            v -= q * u;
            std::mem::swap(&mut x, &mut y);
            std::mem::swap(&mut u, &mut v);
        }
        if y < 0 {
            (-v, -y)
        } else {
            (v, y)
        }
    };
    let b = (g - a * x) / y;
    (a, b)
}