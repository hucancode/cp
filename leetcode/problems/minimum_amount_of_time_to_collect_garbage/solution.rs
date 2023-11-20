impl Solution {
    pub fn garbage_collection(garbage: Vec<String>, mut travel: Vec<i32>) -> i32 {
        let pickup_time: usize = garbage.iter()
            .map(|s| s.len())
            .sum();
        let it = garbage.iter()
            .rev()
            .zip(travel.iter().rev());
        let travel_time_m: i32 = it.clone()
            .skip_while(|(s,t)| !s.contains("M"))
            .map(|(s,t)| t)
            .sum();
        let travel_time_p: i32 = it.clone()
            .skip_while(|(s,t)| !s.contains("P"))
            .map(|(s,t)| t)
            .sum();
        let travel_time_g: i32 = it.clone()
            .skip_while(|(s,t)| !s.contains("G"))
            .map(|(s,t)| t)
            .sum();
        return pickup_time as i32 + 
            travel_time_m + 
            travel_time_p + 
            travel_time_g;
    }
}