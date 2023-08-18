impl Solution {
    pub fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut connected = vec![vec![false;n];n];
        let mut degree = vec![0;n];
        for road in roads {
            let a = road[0] as usize;
            let b = road[1] as usize;
            connected[a][b] = true;
            connected[b][a] = true;
            degree[a] += 1;
            degree[b] += 1;
        }
        let mut ret = 0;
        for i in 0..n {
            for j in i+1..n {
                let rank = degree[i] + degree[j] - if connected[i][j] {1} else {0};
                ret = std::cmp::max(ret, rank);
            }
        }
        return ret;
    }
}