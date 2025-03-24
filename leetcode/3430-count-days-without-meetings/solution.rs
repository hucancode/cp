impl Solution {
    pub fn count_days(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut last = 0;
        let mut ret = 0;
        for m in meetings {
            ret += (m[0] - last - 1).max(0);
            last = last.max(m[1]);
        }
        ret += days - last;
        return ret;
    }
}
