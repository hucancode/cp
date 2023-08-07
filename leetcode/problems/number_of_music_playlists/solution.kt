class Solution {
    fun numMusicPlaylists(n: Int, goal: Int, k: Int): Int {
        val f = Array(goal+1){LongArray(n+1){0}}
        f[0][0] = 1;
        for(i in 1..goal) {
            for(j in 1..n) {
                val addNew = n-j+1
                val replayOld = Math.max(0, j-k)
                f[i][j] += f[i-1][j-1] * addNew
                f[i][j] += f[i-1][j] * replayOld
                f[i][j] %= 1000_000_007L
            }
        }
        return f[goal][n].toInt()
    }
}