impl Solution {
    pub fn color_the_array(n: i32, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = n as usize;
        let mut color = vec![0; n];
        let adj_count = |i, color: &Vec<i32>| {
            let mut ret = 0;
            if i > 0 && color[i-1] == color[i] && color[i] != 0 {
                ret += 1;
            }
            if i < n-1 && color[i+1] == color[i] && color[i] != 0 {
                ret += 1;
            }
            ret
        };
        let mut score = 0;
        queries.iter()
            .map(|arr| {
                let i = arr[0] as usize;
                let c = arr[1];
                let prev = adj_count(i, &color);
                color[i] = c;
                let next = adj_count(i, &color);
                score += next - prev;
                score
            })
            .collect()
    }
}