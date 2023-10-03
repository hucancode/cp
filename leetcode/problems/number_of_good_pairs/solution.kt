class Solution {
    fun numIdenticalPairs(nums: IntArray): Int {
        val group = IntArray(101) {0}
        for(x in nums) group[x]++
        return group.fold(0) { acc,x -> acc+1.rangeUntil(x).sum() }
    }
}