use std::io;

macro_rules! read {
    ($($t:ty),*) => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.split_whitespace();
        ($(
            iter.next().unwrap().parse::<$t>().unwrap()
        ),*)
    }};
}
macro_rules! read_array {
    ($t:ty, $n:expr) => {{
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let iter = input.split_whitespace();
        iter.take($n)
            .map(|item| item.parse::<$t>().unwrap())
            .collect::<Vec<$t>>()
    }};
}
fn solve(mat: Vec<Vec<u32>>, k: usize, d: usize) -> u64 {
    use std::collections::VecDeque;
    let cost = mat
        .iter()
        .map(|depth| {
            let n = depth.len();
            let mut f = vec![0; n];
            f[0] = 1;
            let mut candidates = VecDeque::new();
            candidates.push_back((0, f[0]));
            for i in 1..n {
                while let Some(&(j, _)) = candidates.front() {
                    if i - j <= d + 1 {
                        break;
                    }
                    candidates.pop_front();
                }
                if let Some(&(_, x)) = candidates.front() {
                    let cost = depth[i] + 1;
                    f[i] = x + cost as u64;
                }
                while let Some(&(_, x)) = candidates.back() {
                    if f[i] >= x {
                        break;
                    }
                    candidates.pop_back();
                }
                candidates.push_back((i, f[i]));
            }
            //println!("{f:?}");
            f[n - 1]
        })
        .collect::<Vec<u64>>();
    let n = cost.len();
    let mut cost_prefix = vec![0; n + 1];
    for i in 1..=n {
        cost_prefix[i] = cost_prefix[i - 1] + cost[i - 1];
    }
    (k..=n)
        .map(|i| cost_prefix[i] - cost_prefix[i - k])
        .min()
        .unwrap()
}

fn main() {
    let t = read!(i32);
    for _ in 0..t {
        let (n, m, k, d) = read!(usize, usize, usize, usize);
        let mut mat = Vec::new();
        for _ in 0..n {
            mat.push(read_array!(u32, m));
        }
        println!("{}", solve(mat, k, d));
    }
}
