impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let progress_sum = |first, d, n| (first*2 + d*(n-1))*n/2;
        let week_count = n/7;
        let last_week_day_count = n%7;
        progress_sum(progress_sum(1,1,7),7,week_count) + 
            progress_sum(week_count+1,1,last_week_day_count)
    }
}