impl Solution {
    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let n = max_width as usize;
        let align_both = |line: &Vec<&String>| {
            let m = line.len() - 1;
            let x = n - line.iter().map(|s| s.len()).sum::<usize>();
            let space = x/m;
            let extra = x%m;
            let mut ret = String::new();
            for i in 0..m {
                ret.push_str(line[i]);
                let k = if i < extra {1} else {0};
                for j in 0..space+k {
                    ret.push(' ');
                }
            }
            ret.push_str(line[m]);
            return ret;
        };
        let align_left = |line: &Vec<&String>| {
            let m = line.len() - 1;
            let mut ret = String::new();
            for i in 0..m {
                ret.push_str(line[i]);
                ret.push(' ');
            }
            ret.push_str(line[m]);
            while ret.len() < n {
                ret.push(' ');
            }
            return ret;
        };
        let mut buffer: Vec<&String> = Vec::new();
        let m = words.len();
        let mut i = 0;
        let mut ret = Vec::new();
        while i < m {
            let next = buffer.iter().map(|s| s.len()).sum::<usize>() + buffer.len() + words[i].len();
            if next > n {
                if buffer.len() == 1 {
                    ret.push(align_left(&buffer));
                } else {
                    ret.push(align_both(&buffer));
                }
                buffer.clear();
                continue;
            }
            buffer.push(&words[i]);
            if i == m-1 {
                ret.push(align_left(&buffer));
            }
            i += 1;
        }
        return ret;
    }
}