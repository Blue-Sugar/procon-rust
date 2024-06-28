//!
//! # SlopeTrick
//! 
//! Define a struct of [`SlopeTrick`]
//! 
//! 
//! 

type BinaryHeap<S> = std::collections::BinaryHeap<S>;
const INF: isize = std::isize::MAX >> 1;
/// struct
pub struct SlopeTrick {
    min: isize,
    l: BinaryHeap<isize>,
    r: BinaryHeap<isize>,
}
impl SlopeTrick {
    /// Consturctor of SlopeTrick with f = 0.
    pub fn new() -> Self {
        let mut l = BinaryHeap::new();
        let mut r = BinaryHeap::new();
        l.push(- INF);
        r.push(! INF);
        Self {
            min: 0,
            l,
            r,
        }
    }

    /// Return the minimum.
    pub fn min(&self) -> isize {
        self.min
    }

    /// Add f = c (c: const)
    pub fn add_const(&mut self, c: isize) {
        self.min += c;
    }

    /// Add f(x) = (x - c)+
    pub fn add_plus(&mut self, c: isize) {
        self.min += 0.max(self.l.peek().unwrap() - c);
        self.l.push(c);
        self.r.push(! self.l.pop().unwrap());
    }

    /// Add f(x) = (x - c)-
    pub fn add_minus(&mut self, c: isize) {
        self.min += 0.max(c - ! self.r.peek().unwrap());
        self.r.push(! c);
        self.l.push(! self.r.pop().unwrap());
    }

    /// Add f(x) = |x - c|
    pub fn add_absolute(&mut self, c: isize) {
        self.add_plus(c);
        self.add_minus(c);
    }
}