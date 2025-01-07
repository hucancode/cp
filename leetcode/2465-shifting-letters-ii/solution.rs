impl Solution {
    pub fn shifting_letters(s: String, mut shifts: Vec<Vec<i32>>) -> String {
        use std::collections::{VecDeque, BinaryHeap};
        use std::cmp::Reverse;
        const MOD:i32 = 26;
        let s = s.as_bytes();
        let n = s.len();
        shifts.sort_by(|a,b| a[0].cmp(&b[0]));
        let mut shifts: VecDeque<_> = shifts.into_iter()
            .map(|a| (a[0] as usize, a[1] as usize, a[2]))
            .collect();
        let mut q = BinaryHeap::new();
        let mut offset = 0;
        let mut ret = String::new();
        for i in 0..n {
            while let Some(&(l,r,d)) = shifts.front() {
                if l > i {
                    break;
                }
                shifts.pop_front();
                let d = if d == 0 {-1} else {1};
                q.push((Reverse(r), d));
                offset += d;
                offset = (offset + MOD)%MOD;
            }
            while let Some(&(Reverse(r), d)) = q.peek() {
                if r >= i {
                    break;
                }
                q.pop();
                offset -= d;
                offset = (offset + MOD)%MOD;
            }
            let c = s[i] - 'a' as u8;
            let c = c + offset as u8;
            let c = c%MOD as u8;
            let c = c + 'a' as u8;
            ret.push(c as char);
        }
        ret
    }
}
