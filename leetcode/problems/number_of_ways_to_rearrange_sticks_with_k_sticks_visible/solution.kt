class Solution {
    fun rearrangeSticks(n: Int, k: Int): Int {
        val INF = 1000_000_007L
        val f = Array(n+1) { LongArray(k+1) { 0L } }
        // f[i][j] = how many ways to use exact i sticks, and keep exact j sticks visible
        f[0][0] = 1;
        for(j in 1..k) {
            for(i in j..n) {
                // put first
                f[i][j] += f[i-1][j-1]
                // put back
                f[i][j] += f[i-1][j]*(i - 1)
                f[i][j] = f[i][j].mod(INF)
            }
        }
        return f[n][k].toInt()
    }
}