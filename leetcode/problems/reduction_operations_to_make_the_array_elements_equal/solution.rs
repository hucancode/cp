impl Solution {
    pub fn reduction_operations(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len();
        let mut cost = 0;
        let mut largest = nums[n-1];
        while largest != nums[0] {
            let i = nums.partition_point(|&x| x < largest);
            cost += n-i;
            largest = nums[i-1];
        }
        cost as i32
    }
}