impl Solution {
    pub fn is_path_crossing(path: String) -> bool {
        let mut vis = std::collections::HashSet::new();
        let mut x = 0;
        let mut y = 0;
        vis.insert((0,0));
        for c in path.chars() {
            match c {
                'N' => y += 1,
                'S' => y -= 1,
                'E' => x += 1,
                _ => x -= 1,
            }
            let pos = (x,y);
            if vis.contains(&pos) {
                return true;
            }
            vis.insert(pos);
        }
        false
    }
}