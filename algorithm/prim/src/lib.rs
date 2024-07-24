//! # Prim
//! 
/// function
pub fn prim(n: usize, e: &[(usize, usize, i64)]) -> i64 {
    let _e = e;
    let mut e = vec![vec![]; n];
    for &(v, w, a) in _e.iter() {
        e[v].push((w, a));
        e[w].push((v, a));
    }
    let mut cost = 0;
    let mut heap = std::collections::BinaryHeap::new();
    let mut is_used = vec![false; n];
    for &(w, a) in e[0].iter() {
        heap.push((!a, w));
        is_used[0] = true;
    }
    while let Some((d, v)) = heap.pop() {
        let d = !d;
        if is_used[v] {
            continue;
        }
        cost += d;
        is_used[v] = true;
        for &(w, a) in e[v].iter() {
            heap.push((!a, w));
        }
    }
    cost
}