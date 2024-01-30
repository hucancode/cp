impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut st: Vec<i32> = Vec::new();
        for s in tokens {
            match s.as_str() {
                "+" => {
                    let b = st.pop().unwrap();
                    let a = st.pop().unwrap();
                    st.push(a+b);
                }
                "-" => {
                    let b = st.pop().unwrap();
                    let a = st.pop().unwrap();
                    st.push(a-b);
                }
                "*" => {
                    let b = st.pop().unwrap();
                    let a = st.pop().unwrap();
                    st.push(a*b);
                }
                "/" => {
                    let b = st.pop().unwrap();
                    let a = st.pop().unwrap();
                    st.push(a/b);
                }
                _ => {
                    st.push(s.parse().unwrap());
                }
            }
        }
        st.pop().unwrap()
    }
}