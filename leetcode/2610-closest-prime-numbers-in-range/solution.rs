impl Solution {
    pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
        let mut is_prime = vec![true; right as usize + 1];
        for i in 2..right {
            if !is_prime[i as usize] {
                continue;
            }
            let mut k = 2;
            loop {
                let j = i*k;
                if j > right {
                    break;
                }
                is_prime[j as usize] = false;
                k += 1;
            }
        }
        let primes: Vec<_> = is_prime.into_iter()
            .enumerate()
            .skip(2)
            .filter_map(|(i,x)| if x {Some(i as i32)} else {None})
            .collect();
        let i = primes.partition_point(|&x| x < left);
        let j = primes.partition_point(|&x| x <= right);
        primes[i..j].windows(2)
            .reduce(|a, b| {
                if a[1] - a[0] > b[1] - b[0] {
                    return b;
                }
                return a;
            })
            .map_or(vec![-1,-1], |a| vec![a[0], a[1]])
    }
}
