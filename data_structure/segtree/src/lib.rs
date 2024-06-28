//! #Segtree
//! 
//! Define a struct [`Segtree`] and trait [`Monoid`].
//! 
//! 

use std::ops::RangeBounds;
/// trait of Monoid which is for Segtree
pub trait Monoid {
    /// Value is the set of Monoid
    type Value;
    /// Return identity of Monoid
    fn id() -> Self::Value;
    /// Return the answer of the result of operate lhs and rhs
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
}

/// struct of Segtree
#[derive(Clone)]
pub struct Segtree<M: Monoid> {
    values: Vec<M::Value>,
}
impl<M: Monoid> Segtree<M> {
    /// Constructor of Segtree of Monoid M.
    pub fn new(values: &[M::Value]) -> Self 
    where 
        M::Value: Clone,
    {
        let values_ = values;
        let n = values_.len();
        let mut values = vec![M::id(); n * 2];
        values[n..].clone_from_slice(values_);
        for i in (1..n).rev() {
            values[i] = M::op(&values[i * 2], &values[i * 2 + 1]);
        }
        Self {
            values,
        }
    }

    /// Fold in range.
    pub fn fold<R: RangeBounds<usize>>(&self, range: R) -> M::Value {
        let n = self.values.len() / 2;
        let (mut l, mut r) = open(range, n);
        l += n;
        r += n;
        let mut left = M::id();
        let mut right = M::id();
        while l < r {
            if l % 2 == 1 {
                left = M::op(&left, &self.values[l]);
                l += 1;
            }
            if r % 2 == 1 {
                r -= 1;
                right = M::op(&self.values[r], &right);
            }
            l /= 2;
            r /= 2;
        }
        M::op(&left, &right)
    }

    /// Update to `v` at the index `i`
    pub fn update_at(&mut self, i: usize, v: M::Value) {
        let n = self.values.len() / 2;
        let mut i = i + n;
        self.values[i] = v;
        i /= 2;
        while i > 0 {
            self.values[i] = M::op(&self.values[i * 2], &self.values[i * 2 + 1]);
            i /= 2;
        }
    }

    /// Return the value at index `i`
    pub fn get_at(&self, i: usize) -> M::Value 
    where 
        M::Value: Clone + Copy,
    {
        self.values[i + self.values.len() / 2]
    }

    /// Return the value as a vector. $O(N \log N)$
    pub fn collect(&self) -> Vec<M::Value> 
    where 
        M::Value: Clone + Copy,
    {
        self.values[self.values.len() / 2..].to_vec()
    }
}
impl<M: Monoid> FromIterator<M::Value> for Segtree<M>
where
    M::Value: Clone,
{
    fn from_iter<T: IntoIterator<Item = M::Value>>(iter: T) -> Self {
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