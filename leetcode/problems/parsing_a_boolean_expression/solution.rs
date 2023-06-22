impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut st = Vec::new();
        for c in expression.chars() {
            match c {
                't' | 'f' | '!' | '&' | '|' | '(' => st.push(c),
                ')' => {
                    let mut arr = Vec::new();
                    while let Some(c) = st.pop() {
                        if c == '(' {
                            break;
                        }
                        arr.push(c == 't');
                    }
                    let op = st.pop().unwrap_or('_');
                    let v = match op {
                        '&' => arr.iter().all(|&v|v),
                        '|' => arr.iter().any(|&v|v),
                        _ => arr.iter().all(|&v|!v),
                    };
                    st.push(match v {
                        true => 't',
                        false => 'f',
                    });
                },
                _ => {},
            }
        }
        st.pop().unwrap_or('_') == 't'
    }
}