impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        use std::cmp::Ordering;
        if nums.iter().all(|&x| x == 0) {
            return "0".to_string();
        }
        let mut nums: Vec<String> = nums
            .into_iter()
            .map(|x| x.to_string())
            .collect();
        nums.sort_by(|a, b| {
            let ab = a.chars().chain(b.chars());
            let ba = b.chars().chain(a.chars());
            for (ab,ba) in ab.zip(ba) {
                match ba.cmp(&ab) {
                    Ordering::Equal => continue,
                    diff => return diff,
                }
            }
            return Ordering::Equal;
        });
        //println!("{nums:?}");
        nums.join("")
    }
}
