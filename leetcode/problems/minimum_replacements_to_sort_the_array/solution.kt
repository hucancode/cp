class Solution {
    fun minimumReplacement(nums: IntArray): Long {
        val n = nums.size
        var x = nums[n-1]
        var ret: Long = 0
        for(i in n-2 downTo 0) {
            val k = nums[i]/x + if(nums[i]%x == 0) 0 else 1
            ret += k-1
            x = nums[i]/k
        }
        return ret
    }
}