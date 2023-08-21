class Solution {
    fun minimumFinishTime(tires: Array<IntArray>, changeTime: Int, numLaps: Int): Int {
        val x = tires.map {it[0]}.min() ?: 0
        val INF = x*numLaps + changeTime*(numLaps-1)
        val f = IntArray(numLaps) {INF}
        for(t in tires) {
            var k: Long = t[0].toLong()
            var cost: Long = k
            f[0] = Math.min(f[0], cost.toInt())
            for(i in 1 until numLaps) {
                k *= t[1]
                cost += k
                if(cost >= INF) {
                    break
                }
                f[i] = Math.min(f[i], cost.toInt())
            }
        }
        for(i in 0 until numLaps) {
            for(j in 0 until i) {
                val change = f[i-1-j] + f[j] + changeTime
                f[i] = Math.min(f[i], change)
            }
        }
        return f[numLaps-1];
    }
}