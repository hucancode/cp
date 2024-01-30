use std::collections::BinaryHeap;
use std::cmp::Reverse;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Side {
    Left,
    Right,
}
impl Solution {
    pub fn find_crossing_time(mut n: i32, k: i32, time: Vec<Vec<i32>>) -> i32 {
        let mut waiting: BinaryHeap<(Side, i32, usize)> = BinaryHeap::new(); // side, eff, index
        let mut working: BinaryHeap<(Reverse<i32>, Side, usize)> = BinaryHeap::new(); // time, side, index
        for (i, a) in time.iter().enumerate() {
            let l2r = a[0];
            let r2l = a[2];
            waiting.push((Side::Left, l2r+r2l, i));
        }
        let mut current_time = 0;
        let mut last_arrival_time = 0;
        loop {
            while let Some(&(Reverse(t), side, i)) = working.peek() {
                if t > current_time {
                    break;
                }
                let l2r = time[i][0];
                let r2l = time[i][2];
                //println!("T{current_time}, W{i} with eff {} is waiting at {side}", l2r+r2l);
                waiting.push((side, l2r+r2l, i));
                working.pop();
            }
            if let Some((side, _, i)) = waiting.pop() {
                match side {
                    Side::Right => {
                        let r2l = time[i][2];
                        let put = time[i][3];
                        current_time += r2l;
                        last_arrival_time = current_time;
                        //println!("T{current_time}, W{i} bring a box returned to the left");
                        working.push((Reverse(current_time+put), Side::Left, i));
                    }
                    Side::Left => {
                        if n <= 0 {
                            continue;
                        }
                        let l2r = time[i][0];
                        let pick = time[i][1];
                        current_time += l2r;
                        n -= 1;
                        //println!("T{current_time}, W{i} crossed to the right");
                        working.push((Reverse(current_time+pick), Side::Right, i));
                    }
                }
            } else {
                if let Some(&(Reverse(t), _, _)) = working.peek() {
                    //println!("T{current_time}, wait until T{t}");
                    current_time = t;
                } else {
                    //println!("everyone is done, shipping finished");
                    break;
                }
            }
        }
        return last_arrival_time;
    }
}