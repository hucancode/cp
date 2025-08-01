pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(vec![]);
    }
    use std::collections::HashMap;
    use std::collections::HashSet;
    let n = input.len();
    let mut head: HashMap<u8, HashSet<usize>> = HashMap::new(); 
    let mut tail: HashMap<u8, HashSet<usize>> = HashMap::new(); 
    for (i, &(u,v)) in input.iter().enumerate() {
        head.entry(u)
            .or_default()
            .insert(i);
        tail.entry(v)
            .or_default()
            .insert(i);
    }
    let mut vis = vec![false;n];
    let mut path = Vec::new();
    let mut q = vec![(0, false)];
    while let Some((i, flipped)) = q.pop() {
        if i == n {
            if let Some((i,_)) = path.pop() {
                vis[i] = false;
            }
            continue;
        }
        if vis[i] {
            continue;
        }
        vis[i] = true;
        path.push((i, flipped));
        if path.len() == n {
            let ok = match (path.last(), path.first()) {
                (Some(&(i, false)), Some(&(j, false))) => {
                    let (_, a) = input[i];
                    let (b, _) = input[j];
                    a == b
                }
                (Some(&(i, true)), Some(&(j, false))) => {
                    let (a, _) = input[i];
                    let (b, _) = input[j];
                    a == b
                }
                (Some(&(i, false)), Some(&(j, true))) => {
                    let (_, a) = input[i];
                    let (_, b) = input[j];
                    a == b
                }
                (Some(&(i, true)), Some(&(j, true))) => {
                    let (a, _) = input[i];
                    let (_, b) = input[j];
                    a == b
                }
                _ => false,
            };
            if ok {
                let ret = path.into_iter()
                    .map(|(i, flipped)| {
                        let (a, b) = input[i];
                        if flipped {
                            (b, a)
                        } else {
                            (a, b)
                        }
                    })
                    .collect();
                return Some(ret);
            }
            continue;
        }
        q.push((n, false)); // backtrack
        let (a, b) = input[i];
        let u = if flipped {a} else {b};
        for &i in head.entry(u).or_default().iter() {
            q.push((i, false));
        }
        for &i in tail.entry(u).or_default().iter() {
            q.push((i, true));
        }
    }
    None
}
