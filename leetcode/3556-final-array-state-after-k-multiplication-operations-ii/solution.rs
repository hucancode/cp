impl Solution {
    pub fn get_final_state(nums: Vec<i32>, k: i32, multiplier: i32) -> Vec<i32> {
        if multiplier == 1 {
            return nums;
        }
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;
        let MOD = 1000_000_007;
        let mut k = k as i64;
        let multiplier = multiplier as i64;
        let power = |n: i64, k: i64| {
            let mut result = 1;
            let mut base = n % MOD;
            let mut exp = k;

            while exp > 0 {
                if exp % 2 == 1 {
                    result = (result * base) % MOD;
                }
                base = (base * base) % MOD;
                exp /= 2;
            }

            result
        };
        let mut nums: Vec<_> = nums.into_iter()
            .map(|x| x as i64)
            .collect();
        let mut q: BinaryHeap<(Reverse<i64>, Reverse<usize>)> = nums.iter()
            .enumerate()
            .map(|(i, &x)| (Reverse(x), Reverse(i)))
            .collect();
        let (mut imax, mut xmax) = (0, nums[0]);
        for (i, &x) in nums.iter().enumerate() {
            if x > xmax {
                xmax = x;
                imax = i;
            } else if x == xmax {
                imax = i;
            }
        }
        while k > 0 {
            let (_, Reverse(i)) = q.pop().unwrap();
            nums[i] *= multiplier;
            //println!("multiply nums[{i}] to {}", nums[i]);
            q.push((Reverse(nums[i]), Reverse(i)));
            k -= 1;
            if nums[i] > xmax || (nums[i] == xmax && i >= imax) {
                //println!("multiply nums[{i}] to {next} exceeding max {m}, stop");
                break;
            }
        }
        let n = nums.len() as i64;
        let mut j = 0;
        while let Some((_, Reverse(i))) = q.pop() {
            let p = k/n + if k%n > j {1} else {0};
            nums[i] %= MOD;
            nums[i] *= power(multiplier, p);
            nums[i] %= MOD;
            //println!("multiply nums[{i}] {p} times to {}", nums[i]);
            j += 1;
        }
        nums.into_iter().map(|x| x as i32).collect()
    }
}
