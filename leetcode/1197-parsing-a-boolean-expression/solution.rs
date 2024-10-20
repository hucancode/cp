impl Solution {
    pub fn parse_bool_expr(expression: String) -> bool {
        let mut st = Vec::new();
        for c in expression.chars() {
            match c {
                ',' | '(' => {},
                ')' => {
                    let mut values = Vec::new();
                    //println!("evaluate {st:?}");
                    while let Some(c) = st.pop() {
                        match c {
                            '!' | '&' | '|' => {
                                let v = match c {
                                    '|' => values.iter().any(|&c| c),
                                    '&' => values.iter().all(|&c| c),
                                    _ => values.iter().all(|&c| !c),
                                };
                                //println!("evaluate {values:?}, op = {c} returns {v}");
                                st.push(if v {'t'} else {'f'});
                                break;
                            },
                            c => values.push(c == 't')
                        }
                    }
                },
                c => st.push(c),
            }
        }
        st.pop().is_some_and(|v| v == 't')
    }
}
