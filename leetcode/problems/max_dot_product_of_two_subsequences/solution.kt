class Solution {
    fun maxDotProduct(nums1: IntArray, nums2: IntArray): Int {
        val n = nums1.size
        val m = nums2.size
        val f = Array(n+1) { IntArray(m+1) {-1000_000}}
        var ret = Int.MIN_VALUE
        for(i in 1..n) {
            for(j in 1..m) {
                val x = nums1[i-1]*nums2[j-1]
                f[i][j] = maxOf(
                    x,
                    f[i-1][j-1]+x,
                    f[i-1][j-1],
                    f[i][j-1],
                    f[i-1][j],
                )
                ret = maxOf(ret, f[i][j])
            }
        }
        return ret
    }
}