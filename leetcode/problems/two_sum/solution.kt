class Solution {
    fun twoSum(nums: IntArray, target: Int): IntArray {
        val nums = nums.withIndex().sortedBy { it.value }
        for((i,x) in nums.withIndex()) {
            val (k, x) = x
            val j = nums.binarySearchBy(target-x, i+1) { it.value }
            //println("x = "+x+" find "+(target-x)+", found at "+j)
            if(j >= 0) {
                val (l, y) = nums[j]
                return intArrayOf(k,l)
            }
        }
        return intArrayOf(0,1)
    }
}