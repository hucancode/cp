class Solution {
    fun isMonotonic(nums: IntArray): Boolean {
        return nums.toList()
            .zipWithNext()
            .all { (a,b) -> (a == b) || (a>b) == (nums.first()>nums.last()) }
    }
}