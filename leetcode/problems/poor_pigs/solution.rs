impl Solution {
    pub fn poor_pigs(buckets: i32, minutes_to_die: i32, minutes_to_test: i32) -> i32 {
        let test = 1+minutes_to_test/minutes_to_die;
        (buckets as f32).log(test as f32).ceil() as i32
    }
}