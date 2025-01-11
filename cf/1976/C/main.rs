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

fn solve(code: Vec<u64>, test: Vec<u64>, n: usize, m: usize) {
    let mut coder = Vec::new();
    let mut tester = Vec::new();
    let mut strength = 0;
    for i in 0..n + m + 1 {
        let should_be_coder = code[i] > test[i];
        let can_be_coder = coder.len() < n + 1;
        let can_be_tester = tester.len() < m + 1;
        if !can_be_tester {
            coder.push(i);
            strength += code[i];
            continue;
        }
        if !can_be_coder {
            tester.push(i);
            strength += test[i];
            continue;
        }
        if should_be_coder {
            coder.push(i);
            strength += code[i];
        } else {
            tester.push(i);
            strength += test[i];
        }
    }
    //println!("team coder {coder:?} - tester {tester:?}, strength = {strength}");
    for i in 0..n + m + 1 {
        let mut ret = strength;
        if coder.binary_search(&i).is_ok() {
            ret -= code[i];
            if coder.len() == n {
                if let Some(&j) = tester.last() {
                    ret -= test[j];
                    ret += code[j];
                }
            }
        } else {
            ret -= test[i];
            if tester.len() == m {
                if let Some(&j) = coder.last() {
                    ret -= code[j];
                    ret += test[j];
                }
            }
        }
        print!("{ret} ");
    }
    println!("");
}

fn main() {
    let t = read!(usize);
    for _ in 0..t {
        let (n, m) = read!(usize, usize);
        let code = read_array!(u64, n + m + 1);
        let test = read_array!(u64, n + m + 1);
        solve(code, test, n, m);
    }
}
