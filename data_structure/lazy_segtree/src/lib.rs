//! # LazySegtree
//!
//! Defines a struct [`LazySegtree`] and a trait [`MonoidWithMorphism`] for a lazy segment tree.
//!
//! # Note
//!
//! It does not support binary searches.
//! 
pub mod range_sum_range_affine;


use std::ops::RangeBounds;
use std::mem::replace;
use std::iter::FromIterator;

/// trait for lazy segment tree
pub trait MonoidWithMorphism {
    /// the value type which must be monoid
    type Value;
    /// the set of homomorphism on Value
    type Morphism;
    /// the identity of Value
    fn id() -> Self::Value;
    /// operator in Value as monoid
    fn op(lhs: &Self::Value, rhs: &Self::Value) -> Self::Value;
    /// function which expresses that morphism applies to value
    fn apply(morphism: &Self::Morphism, value: &Self::Value) -> Self::Value;
    /// the identity of Morphism
    fn id_map() -> Self::Morphism;
    /// function which expresses that other morphism composes morphism
    fn compose(morphism: &Self::Morphism, other: &Self::Morphism) -> Self::Morphism;
}

/// struct of lazy segment tree
#[derive(Clone)]
pub struct LazySegtree<M: MonoidWithMorphism> {
    values: Vec<M::Value>,
    morphisms: Vec<M::Morphism>,
}

impl<M: MonoidWithMorphism> LazySegtree<M> {

    /// Constructor a new lazy segment tree from `values: &[M::Value]`.
    pub fn new(values: &[M::Value]) -> Self 
    where
        M::Value: Clone,
        M::Morphism: Clone,
    {
        let values_ = values;
        let n = values_.len();
        assert!(n > 0);
        let mut values = vec![M::id(); 2 * n];
        values[n..].clone_from_slice(values_);
        for i in (1..n).rev() {
            values[i] = M::op(&values[i * 2], &values[i * 2 + 1]);
        }
        Self {
            values,
            morphisms: vec![M::id_map(); 2 * n],
        }
    }

    /// Applies morphism to a range. $O(\log N)$
    pub fn range_apply<R: RangeBounds<usize>>(&mut self, range: R, f: &M::Morphism) {
        let n = self.morphisms.len() / 2;
        let (l, r) = open(range, n); 
        if l == r {
            return;
        }
        let l = n + l;
        let r = n + r;
        for p in (0..usize::BITS - r.leading_zeros()).rev() {
            self.push(l >> p);
            self.push((r - 1) >> p);
        }
        {
            let mut l = l;
            let mut r = r;
            while l < r {
                if l & 1 != 0 {
                    self.morphisms[l] = M::compose(f, &self.morphisms[l]);
                    l += 1;
                }
                if r & 1 != 0 {
                    r -= 1;
                    self.morphisms[r] = M::compose(f, &self.morphisms[r]);
                }
                l >>= 1;
                r >>= 1;
            }
        }
        for p in 1..usize::BITS - r.leading_zeros() {
            self.update(l >> p);
            self.update((r - 1) >> p);
        }
    }

    /// folds a range. $O(\log N)$
    pub fn fold<R: RangeBounds<usize>>(&mut self, range: R) -> M::Value {
        let n = self.morphisms.len() / 2;
        let (mut l, mut r) = open(range, n);
        if l == r {
            return M::id();
        }
        l += n;
        r += n;
        for p in (0..usize::BITS - r.leading_zeros()).rev() {
            self.push(l >> p);
            self.push((r - 1) >> p);
        }
        for p in 1..usize::BITS - r.leading_zeros() {
            self.update(l >> p);
            self.update((r - 1) >> p);
        }
        let mut left = M::id();
        let mut right = M::id();
        while l < r {
            if l & 1 != 0 {
                left = M::op(&left, &M::apply(&self.morphisms[l], &self.values[l]));
                l += 1;
            }
            if r & 1 != 0 {
                r -= 1;
                right = M::op(&M::apply(&self.morphisms[r], &self.values[r]), &right);
            }
            l >>= 1;
            r >>= 1;
        }
        M::op(&left, &right)
    }

    /// Returns the value at the index `i`. $O(\log N)$
    pub fn get(&self, i: usize) -> M::Value 
    where  
        M::Value: Clone,
    {
        let mut i = self.morphisms.len() / 2 + i;
        let mut value = self.values[i].clone();
        while i > 0 {
            value = M::apply(&self.morphisms[i], &value);
            i >>= 1;
        }
        value
    }

    /// Returns the value as a vector. $O(N \log N)$
    pub fn collect(&self) -> Vec<M::Value>
    where
        M::Value: Clone,
    {
        (0..self.morphisms.len() / 2).map(|i| self.get(i)).collect()
    }

    fn push(&mut self, i: usize) {
        let a = replace(&mut self.morphisms[i], M::id_map());
        self.values[i] = M::apply(&a, &self.values[i]);
        if i < self.morphisms.len() / 2 {
            self.morphisms[i << 1] = M::compose(&a, &self.morphisms[i << 1]);
            self.morphisms[i << 1 | 1] = M::compose(&a, &self.morphisms[i << 1 | 1]);
        }
    }
    fn update(&mut self, i: usize) {
        self.values[i] = M::op(
            &M::apply(&self.morphisms[i << 1], &self.values[i << 1]),
            &M::apply(&self.morphisms[i << 1 | 1], &self.values[i << 1 | 1]),
        );
    }
}
impl<M: MonoidWithMorphism> FromIterator<M::Value> for LazySegtree<M>
where
    M::Value: Clone,
    M::Morphism: Clone,
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
#[cfg(test)]
mod tests {
}
