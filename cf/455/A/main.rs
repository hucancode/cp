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

fn solve(mut arr: Vec<i32>) -> i64 {
    let n = arr.len();
    arr.sort();
    let mut f = vec![0; n + 1];
    let mut g = vec![0; n + 1];
    for i in 1..=n {
        let score = arr[i - 1] as i64;
        let target = arr[i - 1] - 1;
        if i > 1 && arr[i - 1] == arr[i - 2] {
            f[i] = f[i - 1] + score;
        } else {
            let j = arr.partition_point(|&x| x < target);
            f[i] = g[j] + score;
        }
        g[i] = g[i - 1].max(f[i]);
    }
    //println!("{g:?}, {f:?}");
    g[n]
}

fn main() {
    let n = read!(usize);
    let arr = read_array!(i32, n);
    println!("{}", solve(arr));
}
