class Solution {
    fun maxCandies(status: IntArray, candies: IntArray, keys: Array<IntArray>, containedBoxes: Array<IntArray>, initialBoxes: IntArray): Int {
        val q = ArrayDeque<Int>()
        val n = status.size
        val haveBox = BooleanArray(n) {false}
        val haveKey = BooleanArray(n) {false}
        val vis = BooleanArray(n) {false}
        var ret = 0
        for(i in initialBoxes) {
            haveBox[i] = true
            if(status[i] == 1) {
                q.add(i)
            }
        }
        while(q.isNotEmpty()) {
            val i = q.removeFirst()
            if(vis[i]) {
                continue
            }
            ret += candies[i]
            vis[i] = true
            for(j in containedBoxes[i]) {
                haveBox[j] = true;
                if(haveKey[j] || status[j] == 1) {
                    q.add(j)
                }
            }
            for(j in keys[i]) {
                haveKey[j] = true
                if(haveBox[j]) {
                    q.add(j)
                }
            }
        }
        return ret
    }
}