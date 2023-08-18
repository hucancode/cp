class Solution {
    fun maximalNetworkRank(n: Int, roads: Array<IntArray>): Int {
        val connected = Array(n) { BooleanArray(n) {false} }
        val degree = IntArray(n) {0}
        for(road in roads) {
            val i = road[0]
            val j = road[1]
            connected[i][j] = true
            connected[j][i] = true
            degree[i]++
            degree[j]++
        }
        var ret = 0
        for(i in 0 until n) {
            for(j in i+1 until n) {
                val rank = degree[i] + degree[j] - if(connected[i][j]) 1 else 0
                ret = Math.max(ret, rank)
            }
        }
        return ret
    }
}