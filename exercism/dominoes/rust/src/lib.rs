pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    use std::collections::HashMap;
    if input.is_empty() {
        return Some(vec![]);
    }
    let n = input.len();
    let mut dot_map: HashMap<u8, Vec<usize>> = HashMap::new();
    for (i, domino) in input.iter().enumerate() {
        dot_map.entry(domino.0).or_default().push(i);
        dot_map.entry(domino.1).or_default().push(i);
    }
    let mut vis = vec![false;n];
    let mut path = Vec::new();
    let mut q = vec![(0, input[0])];
    while let Some((i, current)) = q.pop() {
        if i < 0 {
            path.pop();
            vis[(-i) as usize] = false;
            continue;
        }
        if vis[i as usize] {
            continue;
        }
        vis[i as usize] = true;
        path.push(current);
        if path.len() == n {
            let head = path[0];
            let tail = path[n-1];
            if tail.1 != head.0 {
                continue
            }
            return Some(path.into_iter().collect());
        }
        q.push((-i, (0, 0))); // backtrack
        for &j in dot_map.entry(current.1).or_default().iter() {
            let next = input[j];
            if next.0 == current.1 {
                q.push((j as i32, next));
            } else {
                q.push((j as i32, (next.1,next.0)));
            }
        }
    }
    None
}
