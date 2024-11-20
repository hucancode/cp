impl Solution {
    pub fn take_characters(s: String, k: i32) -> i32 {
        use std::cmp::Ordering;
        let n = s.len();
        let s = s.as_bytes();
        let mut prefix = vec![vec![0;n+1];3];
        for i in 1..=n {
            for j in 0..3 {
                prefix[j][i] = prefix[j][i-1];
            }
            let j = s[i-1] as usize - 'a' as usize;
            prefix[j][i] += 1;
        }
        if prefix[0][n] < k || prefix[1][n] < k || prefix[2][n] < k {
            return -1;
        }
        //println!("{prefix:?}");
        let mut ret = n;
        for i in 0..n {
            let mut pivot = n;
            for j in 0..3 {
                let has = prefix[j][i];
                let need = (k-has).max(0);
                let target = prefix[j][n] - need;
                let p = prefix[j][i..=n]
                    .binary_search_by(|x| match x.cmp(&target) {
                        Ordering::Equal => Ordering::Less,
                        order => order,
                    })
                    .unwrap_err() + i - 1;
                //println!("has {has} need {need} target {target}, keep [{i}, {p}) to satisfy character {j}");
                pivot = pivot.min(p);
            }
            //println!("keep [{i}, {pivot}) to satisfy all characters");
            ret = ret.min(i+n-pivot);
        }
        return ret as i32;
    }
}
