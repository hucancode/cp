impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut st = Vec::new();
        let mut directory = String::new();
        let path = path + "/";
        for c in path.chars() {
            if c == '/' {
                if directory == ".." {
                    st.pop();
                } else if directory != "." && !directory.is_empty() {
                    st.push(directory);
                }
                directory = String::new();
            } else {
                directory.push(c);
            }
        }
        if st.is_empty() {
            return String::from("/");
        }
        let mut ret = String::new();
        for str in st.iter() {
            ret.push('/');
            ret.push_str(str);
        }
        return ret;
    }
}