use std::collections::HashMap;
struct UndergroundSystem {
    card_in: HashMap<i32, (String, i32)>,
    travel_log: HashMap<(String, String), (i64, i32)>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl UndergroundSystem {

    fn new() -> Self {
        Self {
            card_in: HashMap::new(),
            travel_log: HashMap::new(),
        }
    }
    
    fn check_in(&mut self, id: i32, station_name: String, t: i32) {
        self.card_in.insert(id, (station_name, t));
    }
    
    fn check_out(&mut self, id: i32, end_station: String, t: i32) {
        if let Some((start_station, t0)) = self.card_in.get(&id) {
            let key = (start_station.clone(), end_station);
            let delta = t - t0;
            self.travel_log.entry(key)
                .and_modify(|(total_time, count)| {
                    *total_time += delta as i64;
                    *count += 1;
                })
                .or_insert((delta as i64, 1));
        }
    }
    
    fn get_average_time(&self, start_station: String, end_station: String) -> f64 {
        let key = (start_station, end_station);
        if let Some((total_time, count)) = self.travel_log.get(&key) {
            (*total_time as f64)/(*count as f64)
        } else {
            0.0
        }
    }
}

/**
 * Your UndergroundSystem object will be instantiated and called as such:
 * let obj = UndergroundSystem::new();
 * obj.check_in(id, stationName, t);
 * obj.check_out(id, stationName, t);
 * let ret_3: f64 = obj.get_average_time(startStation, endStation);
 */