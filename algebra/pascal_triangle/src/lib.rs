//! # Pascal Triangle
//! 
//! Define a struct [`PascalTriangle`] and trait [`Trait`].
//! Pascal Triangle represents nCr and nHr.
//! 
//! 
/// This trait is for PascalTriangle. It's ring.
mod ring_sample;

/// trait of Ring
pub trait Trait {
    /// Return unit element of product
    fn one() -> Self;
    /// Operator of plus
    fn plus(&self, rhs: Self) -> Self;
}

/// struct of Pascal Triangle.
pub struct PascalTriangle<T> {
    /// max argument
    pub max: usize,
    /// The value which is preserved nCr.
    pub values: Vec<Vec<T>>,
}

impl<T> PascalTriangle<T> 
where
    T: Clone + Copy + Trait,
{
    /// Constructor of Pascal Triangle from max argument. O(max ^ 2)
    pub fn new(max: usize) -> Self {
        let mut values = vec![vec![T::one(); max + 1]; max + 1];
        for i in 1..=max {
            for j in 1..i {
                values[i][j] = values[i - 1][j - 1].plus(values[i - 1][j]);
            }
        }
        Self {
            max,
            values,
        }
    }

    /// Return nCr. O(1).
    pub fn combination(&self, n: usize, r: usize) -> T {
        assert!(n <= self.max);
        assert!(r <= n);
        self.values[n][r]
    }

    /// Return nHr. O(1).
    pub fn duplicated_combination(&self, n: usize, r: usize) -> T {
        assert!(n + r - 1 <= self.max);
        assert!(n <= self.max);
        assert!(r <= self.max);
        self.values[n + r - 1][r]
    }
}