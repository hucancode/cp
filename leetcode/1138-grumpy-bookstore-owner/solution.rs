 impl Solution {
    pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
        use std::collections::VecDeque;
        use std::cmp::max;
        let total: i32 = customers.iter().sum();
        let mut loss = customers.into_iter()
            .zip(grumpy.into_iter())
            .map(|(a,b)| if b == 1 { a } else { 0 });
        let mut q = VecDeque::new();
        let mut evaded = 0;
        let mut total_loss = 0;
        let mut local_loss = 0;
        for x in loss {
            total_loss += x;
            local_loss += x;
            q.push_back(x);
            if(q.len() > minutes as usize) {
                if let Some(y) = q.pop_front() {
                    local_loss -= y;
                }
            }
            evaded = max(evaded, local_loss);
        }
        return total - total_loss + evaded;
    }
}
