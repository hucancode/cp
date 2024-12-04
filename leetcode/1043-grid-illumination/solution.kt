class Solution {
    fun gridIllumination(n: Int, lamps: Array<IntArray>, queries: Array<IntArray>): IntArray {
        val lampByXY = mutableSetOf<Pair<Int, Int>>()
        val lampByRow = mutableMapOf<Int, Int>()
        val lampByCol = mutableMapOf<Int, Int>()
        val lampByDeltaXY = mutableMapOf<Int, Int>()
        val lampByDeltaMXY = mutableMapOf<Int, Int>()

        for (lamp in lamps) {
            val (lx, ly) = lamp
            if (lampByXY.add(Pair(lx, ly))) {
                lampByRow[lx] = lampByRow.getOrDefault(lx, 0) + 1
                lampByCol[ly] = lampByCol.getOrDefault(ly, 0) + 1
                lampByDeltaXY[lx - ly] = lampByDeltaXY.getOrDefault(lx - ly, 0) + 1
                lampByDeltaMXY[-lx - ly] = lampByDeltaMXY.getOrDefault(-lx - ly, 0) + 1
            }
        }

        val result = IntArray(queries.size)

        for ((index, query) in queries.withIndex()) {
            val (qx, qy) = query
            val lit = (lampByRow.getOrDefault(qx, 0) > 0 ||
                    lampByCol.getOrDefault(qy, 0) > 0 ||
                    lampByDeltaXY.getOrDefault(qx - qy, 0) > 0 ||
                    lampByDeltaMXY.getOrDefault(-qx - qy, 0) > 0)

            result[index] = if (lit) 1 else 0

            for (x in qx-1..qx+1) {
                for (y in qy-1..qy+1) {
                    if (lampByXY.remove(Pair(x, y))) {
                        lampByRow[x] = lampByRow.getOrDefault(x, 0) - 1
                        lampByCol[y] = lampByCol.getOrDefault(y, 0) - 1
                        lampByDeltaXY[x - y] = lampByDeltaXY.getOrDefault(x - y, 0) - 1
                        lampByDeltaMXY[-x - y] = lampByDeltaMXY.getOrDefault(-x - y, 0) - 1
                    }
                }
            }
        }
        return result
    }
}
