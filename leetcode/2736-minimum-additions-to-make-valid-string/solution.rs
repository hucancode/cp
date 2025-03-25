impl Solution {
    pub fn add_minimum(word: String) -> i32 {
        let mut cost_a = 1;
        let mut cost_b = 2;
        let mut cost_c = 0;
        for c in word.chars() {
            match c {
                'a' => {
                    cost_a = cost_c;
                    cost_b = cost_a + 1;
                    cost_c = cost_b + 1;
                }
                'b' => {
                    cost_b = cost_a;
                    cost_c = cost_b + 1;
                    cost_a = cost_c + 1;
                }
                _ => {
                    cost_c = cost_b;
                    cost_a = cost_c + 1;
                    cost_b = cost_a + 1;
                }
            }
        }
        return cost_c;
    }
}
