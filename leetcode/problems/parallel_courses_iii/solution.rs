use std::cmp::max;
impl Solution {
    pub fn minimum_time(n: i32, relations: Vec<Vec<i32>>, time: Vec<i32>) -> i32 {
        let n = n as usize;
        let mut f = time.clone();
        let mut in_degree = vec![0;n];
        let mut adj = vec![Vec::new();n];
        for rel in relations {
          let u = rel[0] as usize - 1;
          let v = rel[1] as usize - 1;
          adj[u].push(v);
          in_degree[v] += 1;
        }
        let mut q: Vec<usize> = in_degree.iter()
          .enumerate()
          .filter_map(|(u,&pu)| if pu == 0 {Some(u)} else {None})
          .collect();
        while let Some(u) = q.pop() {
          for &v in adj[u].iter() {
            in_degree[v] -= 1;
            f[v] = max(f[v], f[u]+time[v]);
            if(in_degree[v] == 0) {
              q.push(v);
            }
          }
        }
        return f.into_iter().fold(0, |acc,x| max(acc,x));
    }
}