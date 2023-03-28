use std::cmp::min;
use std::cmp::max;
impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        let mut houses = houses;
        let mut heaters = heaters;
        houses.sort();
        heaters.sort();
        let mut ret = 0;
        for house in houses.iter() {
            let k = match(heaters.binary_search(house)) {
                Ok(x) => heaters[x] - house,
                Err(x) => 
                    if x == heaters.len() {
                        house - heaters[x-1]
                    } else if x == 0 {
                        heaters[x] - house
                    } else {
                        min(house - heaters[x-1], heaters[x] - house)
                    },
            };
            ret = max(ret, k);
        }
        return ret;
    }
}