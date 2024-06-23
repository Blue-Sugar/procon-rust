//! # Galois Field
//! 
//! Define a struct [`GaloisField<P>`]
//! 
//! 
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};


#[macro_export]
macro_rules! gf {
    ($value:expr) => {
        $crate::GaloisField::from($value)
    };
    ($value:expr; mod $p:expr) => {
        $crate::GaloisField::<$p>::from($value)
    };
}

/// Struct which represents value of Galois field Z / pZ.
/// P must be prime number. (no check)
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct GaloisField<const P: u64> {
    /// The value of this instance is `value` % P
    pub value: u64,
}

impl<const P: u64> GaloisField<P> {
    /// Constructor a new Galois field value from `value`.
    pub const fn new(value: u64) -> Self {
        Self { value: value % P }
    }

    /// Take a u64 value less than P from Galois field value.
    pub fn value(&self) -> u64 {
        self.value
    }

    /// Return a value of self to the power of exp. O(\log exp)
    /// Use repeated squaring.
    pub fn pow(&self, mut exp: u64) -> Self {
        let mut res = Self::new(1);
        let mut base = self.clone();
        while exp > 0 {
            if exp & 1 == 1 {
                res *= base;
            }
            base *= base;
            exp >>= 1;
        }
        res
    } 

    /// Return a value of inverse of self. O(\log P)
    /// Use Fermat's little theorem.
    pub fn inv(&self) -> Self {
        assert!(self.value == 0);
        self.pow(P - 2)
    }


}
impl<const P: u64> std::fmt::Display for GaloisField<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value())
    }
}
impl<const P: u64> AddAssign<GaloisField<P>> for GaloisField<P> {
    fn add_assign(&mut self, rhs: GaloisField<P>) {
        self.value += rhs.value;
        if self.value >= P {
            self.value -= P;
        }
    }
}
impl<const P: u64> SubAssign<GaloisField<P>> for GaloisField<P> {
    fn sub_assign(&mut self, rhs: GaloisField<P>) {
        if self.value < rhs.value {
            self.value += P;
        }
        self.value -= rhs.value;
    }
}
impl<const P: u64> MulAssign<GaloisField<P>> for GaloisField<P> {
    fn mul_assign(&mut self, rhs: GaloisField<P>) {
        self.value = self.value * rhs.value % P;
    }
}
impl<const P: u64> DivAssign<GaloisField<P>> for GaloisField<P> {
    fn div_assign(&mut self, rhs: GaloisField<P>) {
        self.value = self.value * rhs.inv().value % P;
    }
}
impl<const P: u64> Neg for GaloisField<P> {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        if self.value > 0 {
            self.value = P - self.value;
        }
        self
    }
}
macro_rules! gf_forward_ops {
    ($(
        $trait:ident,
        $trait_assign:ident,
        $fn:ident,
        $fn_assign:ident,
    )*) => {$(
        impl<const P: u64> $trait_assign<&GaloisField<P>> for GaloisField<P> {
            fn $fn_assign(&mut self, rhs: &GaloisField<P>) {
                self.$fn_assign(*rhs);
            }
        }
        impl<const P: u64, T: Into<GaloisField<P>>> $trait<T> for GaloisField<P> {
            type Output = GaloisField<P>;
            fn $fn(mut self, rhs: T) -> Self::Output {
                self.$fn_assign(rhs.into());
                self
            }
        }
        impl<const P: u64> $trait<&GaloisField<P>> for GaloisField<P> {
            type Output = GaloisField<P>;
            fn $fn(self, rhs: &GaloisField<P>) -> Self::Output {
                self.$fn(*rhs)
            }
        }
        impl<const P: u64, T: Into<GaloisField<P>>> $trait<T> for &GaloisField<P> {
            type Output = GaloisField<P>;
            fn $fn(self, rhs: T) -> Self::Output {
                (*self).$fn(rhs.into())
            }
        }
        impl<const P: u64> $trait<&GaloisField<P>> for &GaloisField<P> {
            type Output = GaloisField<P>;
            fn $fn(self, rhs: &GaloisField<P>) -> Self::Output {
                (*self).$fn(*rhs)
            }
        }
    )*};
}
gf_forward_ops! {
    Add, AddAssign, add, add_assign,
    Sub, SubAssign, sub, sub_assign,
    Mul, MulAssign, mul, mul_assign,
    Div, DivAssign, div, div_assign,
}
macro_rules! impl_from_signed {
    ($($t:ty),*) => {
        $(
            impl<const P: u64> From<$t> for GaloisField<P> {
                fn from(x: $t) -> Self {
                    if x < 0 {
                        - Self::new((P as i64 - x as i64) as u64)
                    } else {
                        Self::new(x as u64)
                    }
                }
            }
        )*
    };
}
impl_from_signed!(i8, i16, i32, i64, i128, isize);
macro_rules! impl_from_unsigned {
    ($($t:ty),*) => {
        $(
            impl<const P: u64> From<$t> for GaloisField<P> {
                fn from(x: $t) -> Self { Self::new(x as u64) }
            }
        )*
    };
}
impl_from_unsigned!(u8, u16, u32, u64, usize);