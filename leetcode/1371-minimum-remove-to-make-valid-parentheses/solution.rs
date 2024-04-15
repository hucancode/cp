impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut ret = String::with_capacity(s.len());
        let mut open = 0;
        let mut close = 0;
        let mut buffer = Vec::with_capacity(s.len());
        for c in s.chars() {
            match c {
                '(' => {
                    open += 1;
                    buffer.push(c);
                }
                ')' => {
                    if open > close {
                        close += 1;
                        buffer.push(c);
                    }
                }
                _ => buffer.push(c)
            }
        }
        open = 0;
        for c in buffer {
           if c == '(' {
                if open < close {
                    ret.push(c);
                    open += 1;
                }
            } else {
                ret.push(c)
            }
        }
        ret
    }
}
