//! # Prime
//!
//! 
//! 
/// function
pub fn is_prime(x: u64) -> bool {
    if x == 0 || x == 1 {
        return false;
    }
    for i in (2..).take_while(|i| i * i <= x) {
        if x % i == 0 {
            return false;
        }
    }
    true
}