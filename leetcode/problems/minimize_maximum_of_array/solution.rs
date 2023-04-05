impl Solution {
    pub fn minimize_array_value(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ret = nums[0];
        let mut k = 0i64;
        for i in 1..n {
            let count = i as i64 + 1;
            if nums[i] < ret {
                k += (ret - nums[i]) as i64;
            } else if nums[i] > ret {
                let mut d = (nums[i] - ret) as i64;
                if d < k {
                    k -= d;
                } else {
                    d -= k;
                    k = 0;
                    let remaining = d%count;
                    ret += (d/count) as i32;
                    if remaining != 0 {
                        ret += 1;
                        k = count - remaining;
                    }
                }
            }
            //println!("avg = {ret}, k = {k}");
        }
        return ret;
    }
}