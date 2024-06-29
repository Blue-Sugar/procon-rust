use superslice::Ext;
/// const 
pub const BUCKET_RATIO: usize = 16;
/// const
pub const SPLIT_RATIO: usize = 24;
/// struct
pub struct SortedMultiSet<T> {
    n: usize,
    values: Vec<Vec<T>>,
}
impl<T> SortedMultiSet<T>
where 
    T: Clone + Copy + Eq + Ord,
{
    /// Constructor of SortedSet by `values: Vec<T>`
    pub fn new(values: &[T]) -> Self {
        let mut values = values.to_vec();
        values.sort();
        let n = values.len();
        let bucket_size = (n as f64 / BUCKET_RATIO as f64).sqrt().ceil() as usize;
        let values = values.chunks(bucket_size)
            .map(|chunk| chunk.to_vec()).collect::<Vec<_>>();
        Self {
            n,
            values,
        }
    }

    /// Return the length of self
    pub fn len(&self) -> usize {
        self.n
    }

    /// Return if self contains `x`
    pub fn contains(&self, value: &T) -> bool {
        let (chunk, _, index) = self.chunk_position(value);
        chunk.len() > 0 && index != chunk.len() && chunk[index] == *value
    }

    /// Return the index of value `value`
    pub fn find(&self, value: &T) -> Option<usize> {
        let (chunk, index_chunk, index) = self.chunk_position(value);
        if chunk.len() == 0 || index == chunk.len() && chunk[index] != *value {
            return None;
        }
        Some(self.values[..index_chunk].iter().map(|v| v.len()).sum::<usize>() + index)
    }

    /// Add `x`. If already exist return false, else add `x` and return true
    pub fn add(&mut self, value: &T) {
        if self.n == 0 {
            self.values = vec![vec![*value]];
            self.n = 1;
        }
        let (mut chunk, index_chunk, index) = self.chunk_position(value);
        self.values[index_chunk].insert(index, *value);
        chunk.insert(index, *value);
        self.n += 1;
        if chunk.len() > self.values.len() * SPLIT_RATIO {
            let mid = chunk.len() >> 1;
            self.values[index_chunk] = chunk[mid..].to_vec();
            self.values.insert(index_chunk, chunk[..mid].to_vec());
        }
    }

    /// Drop the value of `value`
    pub fn drop(&mut self, value: &T) -> bool {
        let (chunk, index_chunk, index) = self.chunk_position(value);
        if chunk.len() == 0 || index == chunk.len() || chunk[index] != *value {
            return false;
        }
        self.values[index_chunk].remove(index);
        if self.values[index_chunk].len() == 0 {
            self.values.remove(index_chunk);
        }
        self.n -= 1;
        true
    }

    /// Remove the value of index `i` and return the value of removed
    pub fn remove(&mut self, mut index: usize) -> Option<T> {
        for (index_chunk, v) in self.values.iter().enumerate() {
            if v.len() <= index {
                index -= v.len();
            } else {
                let res = self.values[index_chunk].remove(index);
                self.n -= 1;
                if self.values[index_chunk].len() == 0 {
                    self.values.remove(index_chunk);
                }
                return Some(res);
            }
        }
        None
    }

    /// Return the nth smallest number.
    pub fn nth(&self, n: usize) -> Option<T> {
        let mut index = 0;
        for v in self.values.iter() {
            if v.len() + index <= n {
                index += v.len();
            } else {
                return Some(v[n - index]);
            }
        }
        None
    }

    /// Return the maximum value less than `value`
    pub fn less_than(&self, value: &T) -> Option<T> {
        for v in self.values.iter().rev() {
            if v[0] < *value {
                let index = v.lower_bound(value);
                return Some(v[index - 1]);
            }
        }
        None
    }

    /// Return the maximum value less than or equal `value`
    pub fn less_than_or_equal(&self, value: &T) -> Option<T> {
        if self.contains(value) {
            return Some(*value);
        }
        self.less_than(value)
    }

    /// Return the minimum value greater than `value`
    pub fn greater_than(&self, value: &T) -> Option<T> {
        for v in self.values.iter() {
            if value < v.last().unwrap() {
                let index = v.upper_bound(value);
                return Some(v[index]);
            }
        }
        None
    }

    /// Return the minimum value greater than or equal `value`
    pub fn greater_than_or_equal(&self, value: &T) -> Option<T> {
        if self.contains(value) {
            return Some(*value);
        }
        self.greater_than(value)
    }

    fn chunk_position(&self, value: &T) -> (Vec<T>, usize, usize) {
        let index_chunk = (0..self.values.len())
            .find(|v| value <= self.values[*v].last().unwrap())
            .unwrap_or(self.values.len() - 1);
        let chunk = self.values[index_chunk].clone();
        let index = chunk.lower_bound(value);
        (chunk, index_chunk, index)
    }
}