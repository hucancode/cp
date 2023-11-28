impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        let mut s = 0;
        let mut p = 0;
        let mut plant_streak = Vec::new();
        for c in corridor.chars() {
            if c == 'S' {
                s += 1;
                if s == 3 {
                    plant_streak.push(p);
                    p = 0;
                    s = 1;
                }
            } else {
                if s == 2 {
                    p += 1;
                }
            }
        }
        if s != 2 {
            return 0;
        }
        plant_streak.into_iter()
            .map(|x| x + 1)
            .fold(1i64, |acc,x| (acc*x)%1000_000_007) as i32
    }
}