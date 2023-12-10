class Solution {
    fun transpose(matrix: Array<IntArray>): Array<IntArray> {
        val n = matrix.size
        val m = matrix[0].size
        val ret = Array(m) { IntArray(n) }
        for(i in 0 until n) {
            for(j in 0 until m) {
                ret[j][i] = matrix[i][j]
            }
        }
        return ret
    }
}