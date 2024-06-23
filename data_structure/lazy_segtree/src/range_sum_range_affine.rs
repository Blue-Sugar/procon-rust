//! # Trait sample range sum range affine range update
//! 
//! Define a enum [`O`] for range sum range affine range update.
//! 


use super::*;

/// enum of morphism in isize
#[derive(Clone, Copy)]
pub enum Morphism {
    /// identity morphism
    Unit,
    /// update to specific value
    Update(isize),
    /// add specific value
    Add(isize),
    /// multiple specific value
    Mul(isize),
    /// transfer bx + a
    Affine(isize, isize),
}

/// enum for implement `MonoidWithMorphism` trait
pub enum O {}

impl MonoidWithMorphism for O {
    type Value = (isize, isize);
    type Morphism = Morphism;
    fn id() -> Self::Value {
        (0, 1)
    }
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value {
        (lhs.0 + rhs.0, lhs.1 + rhs.1)
    }
    fn apply(morphism: &Self::Morphism, value: &Self::Value) -> Self::Value {
        match morphism {
            Morphism::Unit => *value,
            Morphism::Update(x) => (*x * value.1, value.1),
            Morphism::Add(x) => (value.0 + x * value.1, value.1),
            Morphism::Mul(x) => (value.0 * x, value.1),
            Morphism::Affine(x, y) => (value.0 * x + value.1 * y, value.1),
        }
    }
    fn id_map() -> Self::Morphism {
        Morphism::Unit
    }
    fn compose(morphism: &Self::Morphism, other: &Self::Morphism) -> Self::Morphism {
        match morphism {
            Morphism::Unit => *other,
            Morphism::Update(x) => {
                match other {
                    Morphism::Unit => *morphism,
                    Morphism::Update(z) => Morphism::Update(*z),
                    Morphism::Add(z) => Morphism::Update(z + x),
                    Morphism::Mul(z) => Morphism::Update(z * x),
                    Morphism::Affine(z, w) => Morphism::Update(z * x + w),
                }
            }
        Morphism::Add(a) => {
                match other {
                    Morphism::Unit  => *morphism,
                    Morphism::Update(v) => Morphism::Update(*v),
                    Morphism::Add(b) => Morphism::Add(a + b),
                    Morphism::Mul(b) => Morphism::Affine(*b, a * b),
                    Morphism::Affine(b, d) => Morphism::Affine(*b, a * b + d),
                }
            }
            Morphism::Mul(a) => {
                match other {
                    Morphism::Unit => *morphism,
                    Morphism::Update(v) => Morphism::Update(*v),
                    Morphism::Add(b) => Morphism::Affine(*a, *b),
                    Morphism::Mul(b) => Morphism::Mul(a * b),
                    Morphism::Affine(b, d) => Morphism::Affine(a * b, *d),
                }
            }
            Morphism::Affine(a, c) => {
                match other {
                    Morphism::Unit => *morphism,
                    Morphism::Update(v) => Morphism::Update(*v),
                    Morphism::Add(b) => Morphism::Affine(*a, c + b),
                    Morphism::Mul(b) => Morphism::Affine(a * b, b * c),
                    Morphism::Affine(b, d) => Morphism::Affine(a * b, c * b + d),
                }
            }
        }
    }
}