class Solution {
    fun maxSubArray(nums: IntArray): Int {
        var sum = nums[0]
        var ret = sum
        for(x in nums.drop(1)) {
            sum = Math.max(0, sum) + x
            ret = Math.max(ret, sum)
        }
        return ret
    }
}