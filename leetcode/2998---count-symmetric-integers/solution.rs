impl Solution {
    pub fn count_symmetric_integers(low: i32, high: i32) -> i32 {
        let count = |mut n| {
            (11..=n)
                .filter(|&i| {
                    let mut i = i;
                    let mut digits = Vec::new();
                    while i > 0 {
                        digits.push(i%10);
                        i /= 10;
                    }
                    let n = digits.len();
                    n % 2 == 0 && 
                    digits.iter().take(n/2).sum::<i32>() == digits.iter().skip(n/2).sum::<i32>()
                })
                .count() as i32
        };
        return count(high) - count(low-1);
    }
}
