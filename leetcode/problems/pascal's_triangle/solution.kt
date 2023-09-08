class Solution {
    fun generate(numRows: Int): List<List<Int>> {
        val ret = mutableListOf<MutableList<Int>>()
        for(n in 1..numRows) {
            val row = MutableList<Int>(n) {1}
            for(i in 1 until n-1) {
                row[i] = ret[n-2][i] + ret[n-2][i-1]
            }
            ret.add(row)
        }
        return ret
    }
}