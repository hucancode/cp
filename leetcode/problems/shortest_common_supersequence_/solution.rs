#[derive(Clone, Debug)]
enum Pick {
    Only1,
    Only2,
    Both,
}
impl Solution {
    pub fn shortest_common_supersequence(str1: String, str2: String) -> String {
        let str1 = str1.as_bytes();
        let str2 = str2.as_bytes();
        let n = str1.len();
        let m = str2.len();
        let mut f = vec![vec![0;m+1];n+1];
        let mut pick = vec![vec![Pick::Both;m+1];n+1];
        for i in 1..=n {
            pick[i][0] = Pick::Only1;
        }
        for j in 1..=m {
            pick[0][j] = Pick::Only2;
        }
        for i in 1..=n {
            for j in 1..=m {
                if str1[i-1] == str2[j-1] {
                    f[i][j] = f[i-1][j-1] + 1;
                    pick[i][j] = Pick::Both;
                } else if f[i-1][j] > f[i][j-1] {
                    f[i][j] = f[i-1][j];
                    pick[i][j] = Pick::Only1;
                } else {
                    f[i][j] = f[i][j-1];
                    pick[i][j] = Pick::Only2;
                }
            }
        }
        //println!("{pick:?}");
        let mut ret = String::new();
        let mut i = n;
        let mut j = m;
        while i > 0 || j > 0 {
            match pick[i][j] {
                Pick::Only1 => {
                    ret.push(str1[i-1] as char);
                    i -= 1;
                }
                Pick::Only2 => {
                    ret.push(str2[j-1] as char);
                    j -= 1;
                }
                Pick::Both => {
                    ret.push(str1[i-1] as char);
                    i -= 1;
                    j -= 1;
                }
            };
        }
        return ret.chars().rev().collect();
    }
}