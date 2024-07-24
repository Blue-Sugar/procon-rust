//! # Kruskal
//! 
//! 
/// function
pub fn kruskal(n: usize, e: &[(usize, usize, i64)]) -> i64 {
    let mut e = e.to_vec();
    e.sort();
    let mut cost = 0;
    let mut union_find = UnionFind::new(n);
    for &(v, w, a) in e.iter() {
        if union_find.is_same(v, w) {
            continue;
        }
        union_find.unite(v, w);
        cost += a;
    }
    cost
}

pub struct UnionFind {
    values: Vec<isize>,
    count: usize,
}
impl UnionFind {
    /// Constructor of `UnionFind` with all points isolated. 
    pub fn new(n: usize) -> Self {
        Self {
            values: vec![-1; n],
            count : n,
        }
    }

    /// Unite `v` and `w`, return true if v and w is not connected and be connect.
    pub fn unite(&mut self, v: usize, w: usize) -> bool {
        let mut v = self.root(v);
        let mut w = self.root(w);

        if v == w {
            return false;
        }

        if self.values[v] > self.values[w] {
            std::mem::swap(&mut v, &mut w);
        }
        self.values[v] += self.values[w];
        self.values[w] = v as isize;
        self.count -= 1;
        true
    }

    /// Return is same between v and w
    pub fn is_same(&self, v: usize, w: usize) -> bool {
        self.root(v) == self.root(w)
    }

    /// Return the size of set of v.
    pub fn size(&self, v: usize) -> usize {
        - self.values[self.root(v)] as usize
    }

    /// Return how many sets are.
    pub fn count(&self) -> usize {
        self.count
    }

    fn root(&self, mut v: usize) -> usize {
        while self.values[v] >= 0 {
            v = self.values[v] as usize;
        }
        v
    }
}