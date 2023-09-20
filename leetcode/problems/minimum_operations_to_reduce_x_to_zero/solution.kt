class Solution {
    fun minOperations(nums: IntArray, x: Int): Int {
        val n = nums.size
        val f = nums.scan(0) { acc, x -> acc + x }
        if(x > f[n]) {
            return -1
        }
        var ret = n+1
        for((i, target) in f.map {it - x}.withIndex()) {
            val j = f.binarySearchBy(target) {it - f[n]}
            if(j >= 0) {
                ret = Math.min(ret, i+n-j)
            }
        }
        return if(ret <= n) ret else -1
    }
}