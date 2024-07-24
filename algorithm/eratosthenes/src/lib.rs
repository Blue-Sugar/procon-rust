//!
//! # Erathosthenes
//! 
//! Define a struct [`Eratosthenes`]
//! 
/// sturct
pub struct Erathosthenes {
    values: Vec<Option<usize>>,
}
impl Erathosthenes {
    /// Constructor of Ertosthenes to `max_value`
    pub fn new(max_value: usize) -> Self {
        let mut values = (0..=max_value)
            .map(|i| Some(i))
            .collect::<Vec<_>>();
        values[0] = None;
        values[1] = None;
        for base in 2..=max_value {
            if values[base] != Some(base) {
                continue;
            }
            for ratio in (base..).take_while(|ratio| base * ratio <= max_value) {
                if values[base * ratio] == Some(base * ratio) {
                    values[base * ratio] = Some(base);
                } 
            }
        }
        Self {
            values,
        }
    }

    /// Return if value is prime number.
    pub fn is_prime(&self, value: usize) -> bool {
        self.values[value] == Some(value)
    }

    /// Return prime factorization.
    pub fn prime_factorize(&self, mut value: usize) -> Vec<(usize, usize)> {
        let mut res = vec![];
        while value > 0 {
            let factor = self.values[value];
            let mut exp = 0;
            while self.values[value] == factor {
                value /= factor.unwrap();
                exp += 1;
            }
            res.push((factor.unwrap(), exp));
        }
        res
    }

    /// Return the set of dividors
    pub fn dividors(&self, value: usize) -> Vec<usize> {
        let mut res = vec![1];
        let prime_factorize = self.prime_factorize(value);
        for &(factor, exp) in &prime_factorize {
            for i in 0..res.len() {
                let mut v = 1;
                for _ in 0..exp {
                    v *= factor;
                    res.push(res[i] * v);
                }
            }
        }
        res.sort();
        res
    }
}