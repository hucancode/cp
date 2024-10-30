class Solution {
    fun minimumMountainRemovals(nums: IntArray): Int {
        val n = nums.size
        val f = IntArray(n)
        val g = IntArray(n)
        for(i in 1 until n) {
            for(j in 0 until i) {
                if(nums[j] < nums[i]) {
                    f[i] = max(f[i], f[j]+1)
                }
            }
        }
        for(i in n-2 downTo 0) {
            for(j in n-1 downTo i+1) {
                if(nums[j] < nums[i]) {
                    g[i] = max(g[i], g[j]+1)
                }
            }
        }
        return (1 until n-1)
            .filter { f[it] > 0 && g[it] > 0 }
            .map { n-f[it]-g[it]-1}
            .min()
    }
}
