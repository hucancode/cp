class Solution {
    fun averageValue(nums: IntArray): Int {
        return nums.filter { it % 6 == 0 }.average().toInt()
    }
}