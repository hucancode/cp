impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut time: Vec<i32> = dist.iter().zip(speed.iter())
            .map(|(a,b)| a/b + if a%b>0 {1} else {0})
            .collect();
        //println!("time to city {time:?}");
        time.sort();
        //println!("time to city sorted {time:?}");
        let mut ret = 0;
        for (i, &t) in time.iter().enumerate() {
            if i == 0 || i < t as usize {
                ret = i+1;
            } else {
                break;
            }
        }
        return ret as i32;
    }
}