use crate::Trait;

impl Trait for u64 {
    fn one() -> Self {
        1
    }
    fn plus(&self, rhs: Self) -> Self {
        self + rhs
    }
}

use galois_field::GaloisField;
impl<const P: u64> Trait for GaloisField<P> {
    fn one() -> Self {
        Self::new(1)
    }
    fn plus(&self, rhs: Self) -> Self {
        self + rhs
    }
}