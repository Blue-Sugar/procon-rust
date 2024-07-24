//! # Floys Warshall
//! 
const INF: i64 = std::i64::MAX >> 1;
/// function
pub fn floyd_warshall(n: usize, e: &[(usize, usize, i64)], is_direct: bool) -> Vec<Vec<i64>> {
    let mut dist = vec![vec![INF; n]; n];
    for v in 0..n {
        dist[v][v] = 0;
    }
    if is_direct {
        for &(v, w, a) in e.iter() {
            dist[v][w] = a;
        }
    } else {
        for &(v, w, a) in e.iter() {
            dist[v][w] = a;
            dist[w][v] = a;
        }
    }
    for v in 0..n {
        for u in 0..n {
            for w in 0..n {
                dist[u][w] = dist[u][w].min(dist[u][v] + dist[v][w]);
            }
        }
    }
    dist
}

pub fn floyd_warshall_re(n: usize, e: &[(usize, usize, i64)], is_direct: bool) -> Vec<Vec<(i64, Option<usize>)>> {
    let mut dist = vec![vec![(INF, None); n]; n];
    for v in 0..n {
        dist[v][v] = (0, Some(v));
    }
    if is_direct {
        for &(v, w, a) in e.iter() {
            dist[v][w] = (a, Some(v));
        }
    } else {
        for &(v, w, a) in e.iter() {
            dist[v][w] = (a, Some(v));
            dist[w][v] = (a, Some(w));
        }
    }
    for v in 0..n {
        for u in 0..n {
            for w in 0..n {
                if dist[u][w].0 > dist[u][v].0 + dist[v][w].0 {
                    dist[u][w] = (dist[u][v].0 + dist[v][w].0, Some(v));
                }
            }
        }
    }
    dist
}