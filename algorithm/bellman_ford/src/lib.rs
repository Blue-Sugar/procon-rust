//! # Bellman Ford algorithm
//! 
//! 
const INF: i64 = std::i64::MAX >> 1;
/// Bellman Ford
pub fn bellman_ford(n: usize, source: usize, e: &[(usize, usize, i64)], is_direct: bool) -> Vec<i64> {
    let mut dist = vec![INF; n];
    dist[source] = 0;
    if is_direct {
        loop {
            let mut update = false;
            for &(v, w, a) in e.iter() {
                if dist[v] < INF / 2 && dist[v] + a < dist[w] {
                    dist[w] = dist[v] + a;
                    update = true;
                }
            }
            if !update {
                break dist;
            }
        }
    } else {
        loop {
            let mut update = false;
            for &(v, w, a) in e.iter() {
                if dist[v] < INF / 2 && dist[v] + a < dist[w] {
                    dist[w] = dist[v] + a;
                    update = true;
                }
                if dist[w] < INF / 2 && dist[w] + a < dist[v] {
                    dist[v] = dist[w] + a;
                    update = true;
                }
            }
            if !update {
                break dist;
            }
        }
    }
}

pub fn bellman_ford_re(n: usize, source: usize, e: &[(usize, usize, i64)], is_direct: bool) -> Vec<(i64, Option<usize>)> {
    let mut dist = vec![(INF, None); n];
    dist[source] = (0, Some(source));
    if is_direct {
        loop {
            let mut update = false;
            for &(v, w, a) in e.iter() {
                if dist[v].0 < INF / 2 && dist[v].0 + a < dist[w].0 {
                    dist[w] = (dist[v].0 + a, Some(v));
                    update = true;
                }
            }
            if !update {
                break dist;
            }
        }
    } else {
        loop {
            let mut update = false;
            for &(v, w, a) in e.iter() {
                if dist[v].0 < INF / 2 && dist[v].0 + a < dist[w].0 {
                    dist[w] = (dist[v].0 + a, Some(w));
                    update = true;
                }
                if dist[w].0 < INF / 2 && dist[w].0 + a < dist[v].0 {
                    dist[v] = (dist[w].0 + a, Some(w));
                    update = true;
                }
            }
            if !update {
                break dist;
            }
        }
    }
}

pub fn has_negative_cycle(n: usize, source: usize, e: &[(usize, usize, i64)], is_direct: bool) -> bool {
    let mut dist = vec![INF; n];
    dist[source] = 0;
    if is_direct {
        for i in 0..n {
            for &(v, w, a) in e.iter() {
                if dist[v] < INF / 2 && dist[v] + a < dist[w] {
                    dist[w] = dist[v] + a;
                    if i == n - 1 {
                        return true;
                    }
                }
            }
        }
    } else {
        for i in 0..n {
            for &(v, w, a) in e.iter() {
                if dist[v] < INF / 2 && dist[v] + a < dist[w] {
                    dist[w] = dist[v] + a;
                    if i == n - 1 {
                        return true;
                    }
                }
                if dist[w] < INF / 2 && dist[w] + a < dist[v] {
                    dist[v] = dist[w] + a;
                    if i == n - 1 {
                        return true;
                    }
                }
            }
        }
    }
    false
}