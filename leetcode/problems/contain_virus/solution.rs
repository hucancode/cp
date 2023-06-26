use std::collections::HashSet;
impl Solution {
    pub fn contain_virus(mut is_infected: Vec<Vec<i32>>) -> i32 {
        let n = is_infected.len();
        let m = is_infected[0].len();
        let mut vis: Vec<Vec<bool>> = vec![vec![false;m];n];
        let mut ret = 0;
        let dfs = |x: usize, y: usize, v: &mut Vec<Vec<bool>>, is_infected: &Vec<Vec<i32>>| {
            let mut region = Vec::new();
            let mut border = HashSet::new();
            let mut q = Vec::new();
            q.push((x,y));
            let mut wall = 0;
            while let Some((i,j)) = q.pop() {
                if i < 0 || i >= n || j < 0 || j >= m {
                    continue;
                }
                if v[i][j] {
                    continue;
                }
                if is_infected[i][j] == 0 {
                    wall += 1;
                    border.insert((i,j));
                    continue;
                }
                region.push((i,j));
                v[i][j] = true;
                q.push((i+1, j));
                q.push((i, j+1));
                q.push((i-1, j));
                q.push((i, j-1));
            }
            (wall, region, border)
        };
        let mut round = 0;
        loop {
            round += 1;
            let mut best_wall = 0;
            let mut best_region: Vec<(usize, usize)> = Vec::new();
            let mut best_border: HashSet<(usize, usize)> = HashSet::new();
            let mut v = vis.clone();
            let mut infected_last_round = is_infected.clone();
            for i in 0..n {
                for j in 0..m {
                    if infected_last_round[i][j] == 0 || v[i][j] {
                        continue;
                    }
                    let (wall, region, border) = dfs(i,j, &mut v, &infected_last_round);
                    if border.len() > best_border.len() {
                        for (i,j) in best_border {
                            is_infected[i][j] = 1;
                        }
                        best_wall = wall;
                        best_region = region;
                        best_border = border;
                    } else {
                        for (i,j) in border {
                            is_infected[i][j] = 1;
                        }
                    }
                }
            }
            // println!("round {round}, best border = {best_border:?}, best_region = {best_region:?}, to be infected = {tobe_infected:?}");
            if best_wall == 0 {
                break;
            }
            ret += best_wall;
            for (i,j) in best_region {
                vis[i][j] = true;
            }
        }
        return ret;
    }
}