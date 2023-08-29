impl Solution {
    pub fn best_closing_time(customers: String) -> i32 {
        let customers = customers.as_bytes().iter()
            .map(|&x| x == 'Y' as u8)
            .collect::<Vec<bool>>();
        let n = customers.len();
        let mut empty_left = vec![0;n+1];
        let mut customer_right = vec![0;n+1];
        for i in 1..=n {
            empty_left[i] += empty_left[i-1] + if customers[i-1] {0} else {1};
        }
        for i in (0..n).rev() {
            customer_right[i] += customer_right[i+1] + if customers[i] {1} else {0};
        }
        let mut h = 0;
        let mut penalty = empty_left[h] + customer_right[h];
        for i in 1..=n {
            let next = empty_left[i] + customer_right[i];
            if next < penalty {
                penalty = next;
                h = i;
            }
        }
        return h as i32;
    }
}