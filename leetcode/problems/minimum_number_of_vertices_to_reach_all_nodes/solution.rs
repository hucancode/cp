impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        let mut f = vec![true; n as usize];
        edges.into_iter()
            .map(|e| e[1] as usize)
            .for_each(|i| f[i] = false);
        f.into_iter()
            .enumerate()
            .filter(|&(_, x)| x)
            .map(|(i, _)| i as i32)
            .collect()
    }
}