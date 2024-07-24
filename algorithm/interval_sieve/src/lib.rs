//! # Interval sieve
//! 

use std::ops::RangeBounds;
/// function
pub struct IntervalSeive {
    value: Vec<bool>,
    min_value: usize,
}
impl IntervalSeive {
    /// Constructor
    pub fn new<R: RangeBounds<usize>>(range: R) -> IntervalSeive {
        let (l, r) = open(range);
        let mut value = vec![true; r - l];
        let mut small = (0..)
            .take_while(|i| i * i <= r)
            .map(|_| true).collect::<Vec<_>>();
        for i in (2..).take_while(|i| i * i <= r) {
            if small[i] {
                for j in (2..).take_while(|j| j * j <= r) {
                    small[j] = false;
                }
                for j in 2.max((l + i - 1) / i * i)..=r {
                    value[j - l] = false;
                }
            }
        }
        Self {
            value,
            min_value: l,
        }
    }

    /// is prime
    pub fn is_prime(&self, x: usize) -> bool {
        self.value[x - self.min_value]
    }
}
fn open<R: RangeBounds<usize>>(range: R) -> (usize, usize) {
    use std::ops::Bound;
    let start = match range.start_bound() {
        Bound::Unbounded => 0,
        Bound::Included(&x) => x,
        Bound::Excluded(&x) => x + 1,
    };
    let end = match range.end_bound() {
        Bound::Unbounded => panic!("あちゃ"),
        Bound::Included(&x) => x + 1,
        Bound::Excluded(&x) => x,
    };
    (start, end)
}