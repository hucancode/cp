use std::collections::BinaryHeap;
use std::cmp::Reverse;
impl Solution {
    pub fn most_booked(n: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut busy_room = BinaryHeap::new();
        let mut free_room = BinaryHeap::new();
        for i in 0..n {
            free_room.push(Reverse(i));
        }
        let mut used_count = vec![0; n];
        meetings.sort_by(|a,b| a[0].cmp(&b[0]));
        //println!("meetings = {meetings:?}");
        for meeting in meetings {
            let start = meeting[0] as i64;
            let end = meeting[1] as i64;
            //println!("arrange meeting from {start} until {end}");
            while let Some(&(Reverse(time), Reverse(r))) = busy_room.peek() {
                if time > start {
                    break;
                }
                busy_room.pop();
                free_room.push(Reverse(r));
                //println!("room {r} is free");
            }
            if let Some(Reverse(r)) = free_room.pop() {
                busy_room.push((Reverse(end), Reverse(r)));
                used_count[r] += 1;
                //println!("use room {r}, use count = {}", used_count[r]);
            } else {
                if let Some((Reverse(time), Reverse(r))) = busy_room.pop() {
                    let delta = time - start;
                    busy_room.push((Reverse(end + delta), Reverse(r)));
                    used_count[r] += 1;
                    //println!("wait {delta} and use room {r}, use count = {}", used_count[r]);
                }
            }
        }
        let mut r = 0;
        for i in 1..n {
            if used_count[i] > used_count[r] {
                r = i;
            }
        }
        return r as i32;
    }
}