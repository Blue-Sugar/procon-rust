//! # Dijkstra
//! 
//! 
const INF: u64 = std::u64::MAX >> 1;
/// function
pub fn dijkstra(n: usize, source: usize, e: &[(usize, usize, u64)], is_direct: bool) -> Vec<u64> {
    let _e = e;
    let mut e = vec![vec![]; n];
    for &(v, w, a) in _e.iter() {
        e[v].push((w, a));
    }
    if !is_direct {
        for &(v, w, a) in _e.iter() {
            e[w].push((v, a));
        }
    }
    let mut dist = vec![INF; n];
    dist[source] = 0;
    let mut heap = std::collections::BinaryHeap::new();
    heap.push((!0, source));
    while let Some((d, v)) = heap.pop() {
        let d = !d;
        if dist[v] < d {
            continue;
        }
        for &(w, dd) in e[v].iter() {
            if dist[v] + dd < dist[w] {
                dist[w] = dist[v] + dd;
                heap.push((!(dist[v] + dd), w));
            }
        }
     }
    dist
}

pub fn dijkstra_re(n: usize, source: usize, e: &[(usize, usize, u64)], is_direct: bool) -> Vec<(u64, Option<usize>)> {
    let _e = e;
    let mut e = vec![vec![]; n];
    for &(v, w, a) in _e.iter() {
        e[v].push((w, a));
    }
    if !is_direct {
        for &(v, w, a) in _e.iter() {
            e[w].push((v, a));
        }
    }
    let mut dist = vec![(INF, None); n];
    dist[source] = (0, Some(source));
    let mut heap = std::collections::BinaryHeap::new();
    heap.push((!0, source));
    while let Some((d, v)) = heap.pop() {
        let d = !d;
        if dist[v].0 < d {
            continue;
        }
        for &(w, dd) in e[v].iter() {
            if dist[v].0 + dd < dist[w].0 {
                dist[w] = (dist[v].0 + dd, Some(v));
                heap.push((!(dist[v].0 + dd), w));
            }
        }
     }
    dist
}