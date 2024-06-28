//! # Fenwick Tree
//! 
//! Define a struct [`FenwickTree`] and trait [`AbelianGroup`]
//! 

use std::ops::RangeBounds;
/// trait for FenwickTree
pub trait AbelianGroup {
    /// the set of AbelianGroup
    type Value;
    /// Return identity of AbelianGroup
    fn id() -> Self::Value;
    /// Return the answer of operate lhs and rhs
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    /// Return the answer of inverse value
    fn inv(value: &Self::Value) -> Self::Value;
}

/// struct of FenwickTree
pub struct FenwickTree<G: AbelianGroup> {
    values: Vec<G::Value>,
}
impl<G: AbelianGroup> FenwickTree<G> {
    /// Constructor of FenwickTree by `values: &[G::Value]`
    pub fn new(values: &[G::Value]) -> Self 
    where
        G::Value: Clone,
    {
        let values_ = values;
        let mut values = vec![G::id(); values_.len()];
        for (i, value) in values_.iter().enumerate() {
            let mut i = i + 1;
            while i <= values_.len() {
                values[i - 1] = G::op(&values[i - 1], value);
                i += i & (!i + 1);
            }
        }
        Self {
            values,
        }
    }

    // Return fold of range.
    pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> G::Value {
        let (mut l, mut r) = open(range, self.values.len());
        let mut left = G::id();
        let mut right = G::id();
        while l > 0 {
            left = G::op(&left, &self.values[l - 1]);
            l -= l & (!l + 1);
        }
        while r > 0 {
            right = G::op(&self.values[r - 1], &right);
            r -= r & (!r + 1);
        }
        G::op(&right, &G::inv(&left))
    }
}
impl<G:AbelianGroup> FromIterator<G::Value> for FenwickTree<G>
where
    G::Value: Clone,
{
    fn from_iter<T: IntoIterator<Item = G::Value>>(iter: T) -> Self {
        Self::new(&iter.into_iter().collect::<Vec<_>>())
    }
}
fn open<R: RangeBounds<usize>>(range: R, n: usize) -> (usize, usize) {
    use std::ops::Bound;
    let start = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
    };
    let end = match range.end_bound() {
        Bound::Unbounded => n,
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
    };
    (start, end)
}