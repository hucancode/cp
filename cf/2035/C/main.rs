// UNSOLVED
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

fn solve_odd(n: i32) -> (i32, Vec<i32>) {
    return (n, vec![1, 2, 3, 4, 5]);
}
fn solve_even(n: i32) -> (i32, Vec<i32>) {
    return (n, vec![1, 2, 3, 4, 5]);
}
fn solve(n: i32) -> (i32, Vec<i32>) {
    if n < 5 {
        let mut k = 0;
        let mut v = Vec::new();
        for i in 1..=n {
            v.push(i);
            if i % 2 == 0 {
                k |= i
            } else {
                k &= i
            };
        }
        return (k, v);
    }
    let pool: Vec<i32> = (1..=18).map(|i| [(1 << i) - 1, 1 << i]).flatten().collect();
    println!("{pool:?}");
    if n % 2 == 0 {
        solve_even(n)
    } else {
        solve_odd(n)
    }
}

fn main() {
    let t = read!(i32);
    for _ in 0..t {
        let n = read!(i32);
        let (k, p) = solve(n);
        let p = p
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{k}");
        println!("{p}");
    }
}
