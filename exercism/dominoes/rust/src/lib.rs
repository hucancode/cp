pub fn chain(input: &[(u8, u8)]) -> Option<Vec<(u8, u8)>> {
    if input.is_empty() {
        return Some(vec![]);
    }
    use std::collections::HashMap;
    let n = input.len();
    let mut head: HashMap<u8, Vec<usize>> = HashMap::new(); 
    let mut tail: HashMap<u8, Vec<usize>> = HashMap::new(); 
    for (i, &(u,v)) in input.iter().enumerate() {
        head.entry(u)
            .or_default()
            .push(i);
        tail.entry(v)
            .or_default()
            .push(i);
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
            let ok = match path.last() {
                Some(&(i, flipped)) => {
                    let (a, b) = input[i];
                    let (c, d) = input[0];
                    let u = if flipped {a} else {b};
                    let v = c;
                    u == v
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
