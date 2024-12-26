impl Solution {
    pub fn add_operators(num: String, target: i32) -> Vec<String> {
        const MERGE: i32 = 0b00;
        const MUL: i32 = 0b01;
        const MINUS: i32 = 0b10;
        const PLUS: i32 = 0b11;
        let num: Vec<i64> = num.chars()
            .map(|c| c as i64 - '0' as i64)
            .collect();
        let n = num.len() - 1;
        let mut ret = Vec::new();
        let evaluate = |mask: i32| {
            let mut e = Vec::new();
            let mut op = Vec::new();
            e.push(num[0]);
            for i in 0..n {
                let this_op = (mask>>(i*2)) & 0b11;
                match this_op {
                    MUL => {
                        e.push(num[i+1]);
                        op.push(this_op);
                    }
                    MINUS | PLUS => {
                        while let Some(prev_op) = op.pop() {
                            let b = e.pop().unwrap_or(0);
                            let a = e.pop().unwrap_or(0);
                            e.push(match prev_op {
                                MUL => a*b,
                                MINUS => a-b,
                                PLUS => a+b,
                                _ => 0,
                            });
                        }
                        op.push(this_op);
                        e.push(num[i+1]);
                    }
                    _ => if let Some(x) = e.pop() {
                        if x == 0 {
                            return None;
                        }
                        e.push(x*10 + num[i+1]);
                    }
                }
            }
            while let Some(prev_op) = op.pop() {
                let b = e.pop().unwrap_or(0);
                let a = e.pop().unwrap_or(0);
                e.push(match prev_op {
                    MUL => a*b,
                    MINUS => a-b,
                    PLUS => a+b,
                    _ => 0,
                });
            }
            return Some(e.pop().unwrap_or(0));
        };
        let print = |mask:i32| {
            let mut ret = String::new();
            ret.push((num[0] as u8 + '0' as u8) as char);
            for i in 0..n {
                let this_op = (mask>>(i*2)) & 0b11;
                match this_op {
                    MUL => ret.push('*'),
                    PLUS => ret.push('+'),
                    MINUS => ret.push('-'),
                    _ => {},
                };
                ret.push((num[i+1] as u8 + '0' as u8) as char);
            }
            return ret;
        };
        let m = 1<<(n*2);
        let target = target as i64;
        for i in 0..m {
            if evaluate(i).is_some_and(|x| x == target) {
                ret.push(print(i));
            }
        }
        return ret;
    }
}
