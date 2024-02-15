use std::cmp::min;
impl Solution {
    pub fn min_operations_to_flip(expression: String) -> i32 {
        let mut op_st = Vec::new();
        let mut value = Vec::new();
        let mut invert_cost = Vec::new();
        for c in expression.chars() {
            //println!("check {c}");
            let mut should_eval = false;
            match c {
                '1' | '0' => {
                    value.push(c as u8 - '0' as u8);
                    invert_cost.push(1);
                    should_eval = true;
                },
                ')' => {
                    op_st.pop();
                    should_eval = true;
                },
                _ => {
                    op_st.push(c);
                },
            }
            if value.len() < 2 || !should_eval {
                //println!("value {value:?} invert_cost {invert_cost:?}, op {op_st:?}");
                continue;
            }
            if let Some(&op) = op_st.last() {
                if op == '|' || op == '&' {
                    op_st.pop();
                    let a = value.pop().unwrap();
                    let b = value.pop().unwrap();
                    let invert_a = invert_cost.pop().unwrap();
                    let invert_b = invert_cost.pop().unwrap();
                    if op == '|' {
                        value.push(a|b);
                    } else {
                        value.push(a&b);
                    }
                    if a != b {
                        invert_cost.push(1);
                    } else if (a == 0 && op == '|') || (a == 1 && op == '&') {
                        invert_cost.push(min(invert_a, invert_b));
                    } else {
                        invert_cost.push(min(invert_a, invert_b)+1);
                    }
                }
            }
            //println!("value {value:?} invert_cost {invert_cost:?}, op {op_st:?}");
        }
        invert_cost.pop().unwrap()
    }
}