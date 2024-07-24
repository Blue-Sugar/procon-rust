//! # Repeated Squaring
//! 
/// function
pub fn mod_pow(x: u64, mut n: u64, m: u64) -> u64 {
    let mut res = 1;
    let mut base = x;
    while n > 0 {
        if n & 1 == 1 {
            res *= base;
            res %= m;
        }
        base *= base;
        base %= base;
        n >>= 1;
    }
    res
}