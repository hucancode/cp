impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let n = s.len();
        if n <= 1 {
            return false;
        }
        let s = s.as_bytes();
        let mut primes = vec![true;n/2+1];
        for i in 2..n/2 {
            if !primes[i] {
                continue;
            }
            let mut j = 2;
            while i*j <= n/2 {
                primes[i*j] = false;
                j+= 1;
            }
        }
        primes.into_iter()
            .enumerate()
            .skip(2)
            .filter_map(|(i,x)| if x {Some(i)} else {None})
            .filter(|i| n%i == 0)
            .chain(std::iter::once(n))
            .any(|x| {
                let len = n/x;
                (0..len).all(|i| 
                    (1..x).all(|j| 
                        s[i] == s[i+j*len]
                    )
                )
            })
    }
}