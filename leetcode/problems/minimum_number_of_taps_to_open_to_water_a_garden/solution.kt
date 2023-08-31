class Solution {
    fun minTaps(n: Int, ranges: IntArray): Int {
        val f = IntArray(n+1) {n+1}
        f[0] = 0;
        for((i, range) in ranges.withIndex()) {
            val a = Math.max(0, i - range)
            val b = Math.min(n, i + range)
            for(i in a+1..b) {
                f[i] = Math.min(f[i], f[a] + 1)
            }
        }
        //println(f.joinToString())
        return if(f[n] > n) -1 else f[n]
    }
}