use std::collections::HashSet;
impl Solution {
    pub fn is_escape_possible(blocked: Vec<Vec<i32>>, source: Vec<i32>, target: Vec<i32>) -> bool {
        let INF: i32 = 1000_000 - 1;
        let mut n = blocked.len();
        let mut blocks: HashSet<(i32, i32)> = blocked.into_iter()
            .map(|a| (a[0], a[1]))
            .collect();
        let max_area = n*(n - 1)/2;
        let mut q = Vec::new();
        q.push((source[0], source[1]));
        let mut step = 0;
        let mut source_vis = HashSet::new();
        while let Some((x, y)) = q.pop() {
            if x < 0 || x > INF || y < 0 || y > INF || 
                blocks.contains(&(x, y)) || 
                !source_vis.insert((x, y)) {
                    continue;
                }
            if x == target[0] && y == target[1] {
                return true;
            }
            step += 1;
            q.push((x-1, y));
            q.push((x+1, y));
            q.push((x, y-1));
            q.push((x, y+1));
            if step > max_area {
                break;
            }
        }
        step = 0;
        if(!q.is_empty()) {
            q.clear();
            q.push((target[0], target[1]));
        }
        let mut target_vis = HashSet::new();
        while let Some((x, y)) = q.pop() {
            if x < 0 || x > INF || y < 0 || y > INF || 
                blocks.contains(&(x, y)) || 
                !target_vis.insert((x, y)) {
                    continue;
                }
            if source_vis.contains(&(x, y)) {
                return true;
            }
            step += 1;
            q.push((x-1, y));
            q.push((x+1, y));
            q.push((x, y-1));
            q.push((x, y+1));
            if step > max_area {
                break;
            }
        }
        !q.is_empty()
    }
}