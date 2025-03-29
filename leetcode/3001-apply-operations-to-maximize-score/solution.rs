const LIMIT: usize = 100_000;
const PRIMES_UNDER_LIMIT: usize = 9592;

const fn sieve() -> [bool; LIMIT + 1] {
    let mut is_prime = [true; LIMIT + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    
    let mut p = 2;
    while p * p <= LIMIT {
        if is_prime[p] {
            let mut i = p * p;
            while i <= LIMIT {
                is_prime[i] = false;
                i += p;
            }
        }
        p += 1;
    }
    is_prime
}

const fn generate_prime_list() -> [usize; PRIMES_UNDER_LIMIT] {
    let is_prime = sieve();
    let mut primes = [0; PRIMES_UNDER_LIMIT];
    let mut index = 0;
    let mut i = 2;
    while i <= LIMIT {
        if is_prime[i] {
            primes[index] = i;
            index += 1;
        }
        i += 1;
    }
    primes
}

const PRIMES: [usize; PRIMES_UNDER_LIMIT] = generate_prime_list();

fn count_prime_factors(k: &i32) -> usize {
    let mut k = *k as usize;
    let mut count = 0;
    for &p in PRIMES.iter() {
        if p * p > k {
            break;
        }
        if k % p == 0 {
            count += 1;
            while k % p == 0 {
                k /= p;
            }
        }
    }
    if k > 1 {
        count += 1; // k itself is prime
    }
    count
}

const MOD: i64 = 1_000_000_007;
fn power(x: i64, n: i64) -> i64 {
    if n == 0 {
        return 1;
    }
    if n == 1 {
        return x % MOD;
    }
    let k = power(x, n/2);
    return (k * k % MOD) * power(x, n%2) % MOD;
}

impl Solution {
    pub fn maximum_score(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::BinaryHeap;
        let n = nums.len();
        let scores: Vec<_> = nums.iter().map(count_prime_factors).collect();
        let mut cover_l = vec![1;n]; // cover_l[i] = how many item nums[i] can cover until it meet a number with equal or higher score to the left
        let mut cover_r = vec![1;n]; // cover_r[i] = how many item nums[i] can cover until it meet a number with higher score to the right
        let mut st = Vec::new();
        for i in 0..n {
            while let Some(&j) = st.last() {
                if scores[j] >= scores[i] {
                    break;
                }
                st.pop();
            }
            cover_l[i] = st.last().map_or(i + 1, |j| i - j);
            st.push(i);
        }
        let mut st = Vec::new();
        for i in (0..n).rev() {
            while let Some(&j) = st.last() {
                if scores[j] > scores[i] {
                    break;
                }
                st.pop();
            }
            cover_r[i] = st.last().map_or(n-i, |j| j - i);
            st.push(i);
        }
        let contributions: Vec<_> = (0..n).map(|i| cover_l[i]*cover_r[i]).collect();
        //println!("scores {scores:?}, cover left {cover_l:?}, cover right {cover_r:?}, contributions {contributions:?}");
        let mut q: BinaryHeap<_> = nums.into_iter()
            .zip(contributions.into_iter())
            .collect();
        let mut ret = 1;
        let mut k = k as usize;
        while let Some((x, count)) = q.pop() {
            //println!("take {x} with {count} contribution");
            ret *= power(x as i64, count.min(k) as i64);
            ret %= MOD;
            k -= count.min(k);
            if k == 0 {
                break;
            }
        }
        return ret as i32;
    }
}
