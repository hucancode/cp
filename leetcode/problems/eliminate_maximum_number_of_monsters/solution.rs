impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        let mut time: Vec<i32> = dist.iter().zip(speed.iter())
            .map(|(a,b)| 1+(a-1)/b)
            .collect();
        //println!("time to city {time:?}");
        time.sort();
        //println!("time to city sorted {time:?}");
        time.into_iter()
          .enumerate()
          .skip(1)
          .take_while(|&(i,t)| i < t as usize)
          .last()
          .map_or(1, |(i,_)| i as i32 + 1) 
    }
}