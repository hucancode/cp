impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let sum = apple.iter().sum();
        capacity.sort();
        let mut ret = 0;
        let mut cap = 0;
        while let Some(x) = capacity.pop() {
            cap += x;
            ret += 1;
            if cap >= sum {
                break;
            }
        }
        ret
    }
}
