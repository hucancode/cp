class Solution {
    fun maximalRectangle(matrix: Array<CharArray>): Int {
        val n = matrix.size;
        val m = matrix[0].size
        val rowStreak = Array(n) { Array(m) {0} }
        for(i in 0 until n) {
            if(matrix[i][0] == '1') rowStreak[i][0] = 1
            for(j in 1 until m) {
                if(matrix[i][j] != '1') continue
                rowStreak[i][j] = rowStreak[i][j-1] + 1
            }
        }
        var ret = 0
        for(i in 0 until n) {
            for(j in 0 until m) {
                var h = 0
                var w = rowStreak[i][j]
                while ((i - h >= 0) and (w > 0)) {
                    w = Math.min(w, rowStreak[i-h][j])
                    ret = Math.max(ret, w*(h+1))
                    h++
                }
            }
        }
        return ret
    }
}