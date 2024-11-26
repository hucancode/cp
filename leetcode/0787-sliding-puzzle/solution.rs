impl Solution {
    pub fn sliding_puzzle(board: Vec<Vec<i32>>) -> i32 {
        use std::collections::{HashSet, VecDeque};
        let n = board.len();
        let m = board[0].len();
        let k = 1<<(m*n*3);
        let ret: i32 = (1..6).map(|x| x<<((x-1)*3)).sum();
        let swap = |mask, i, j| {
            let i = i*3;
            let j = j*3;
            let delta = j-i;
            let take_i: i32 = 0b111<<i;
            let take_j: i32 = 0b111<<j;
            let mi: i32 = (mask & take_i)<<delta;
            let mj: i32 = (mask & take_j)>>delta;
            mask & !(take_i | take_j) | mi | mj
        };
        let mut vis = HashSet::new();
        let mut mask: i32 = 0;
        let mut zero_slot = 0;
        for i in (0..n).rev() {
            for j in (0..m).rev() {
                mask = (mask<<3) | board[i][j];
                if board[i][j] == 0 {
                    zero_slot = i*m+j;
                }
            }
        }
        let mut q = VecDeque::new();
        q.push_back((mask, zero_slot, 0));
        while let Some((mask, zero_slot, score)) = q.pop_front() {
            if !vis.insert(mask) {
                continue;
            }
            if mask == ret {
                return score;
            }
            if zero_slot >= m {
                let mask = swap(mask, zero_slot - m, zero_slot);
                q.push_back((mask, zero_slot - m, score + 1));
            }
            if zero_slot < n*m-m {
                let mask = swap(mask, zero_slot, zero_slot+m);
                q.push_back((mask, zero_slot+m, score + 1));
            }
            if zero_slot%m > 0 {
                let mask = swap(mask, zero_slot - 1, zero_slot);
                q.push_back((mask, zero_slot - 1, score + 1));
            }
            if zero_slot%m < m-1 {
                let mask = swap(mask, zero_slot, zero_slot+1);
                q.push_back((mask, zero_slot + 1, score + 1));
            }
        }
        return -1;
    }
}
