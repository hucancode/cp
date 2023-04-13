impl Solution {
    pub fn validate_stack_sequences(pushed: Vec<i32>, popped: Vec<i32>) -> bool {
        let mut popped = popped;
        popped.reverse();
        let mut f = Vec::new();
        for x in pushed {
            f.push(x);
            while let (Some(&x), Some(&y)) = (popped.last(), f.last()) {
                if x == y {
                    f.pop();
                    popped.pop();
                } else {
                    break;
                }
            }
        }
        return popped.len() == f.len() && 
            popped.iter().zip(f.iter()).all(|(x, y)| x == y);
    }
}