class Solution {
    fun uniquePathsWithObstacles(grid: Array<IntArray>): Int {
        val n = grid.size
        val m = grid[0].size
        val f = Array(n) { IntArray(m) {0} }
        for(i in 0 until n) if(grid[i][0] == 0) f[i][0] = 1 else break
        for(j in 0 until m) if(grid[0][j] == 0) f[0][j] = 1 else break
        for(i in 1 until n) {
            for(j in 1 until m) {
                if(grid[i][j] == 0) {
                    f[i][j] = f[i-1][j] + f[i][j-1]
                }
            }
        }
        return f[n-1][m-1]
    }
}