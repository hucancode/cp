class Solution {
    fun sortArrayByParity(nums: IntArray): IntArray {
        return nums.sortedBy { it%2 }.toIntArray()
    }
}