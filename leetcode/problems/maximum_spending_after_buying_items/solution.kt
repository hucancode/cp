class Solution {
    fun maxSpending(values: Array<IntArray>): Long {
        val values = values.map { it.toMutableList() }
        val q = PriorityQueue<Pair<Int,Int>>(){ a, b -> (a.first - b.first).sign }
        var ret = 0L
        for(i in values.indices) {
            q.add(Pair(values[i].removeLast(), i))
        }
        var day = 1L
        while(q.isNotEmpty()) {
            val (x, i) = q.remove()
            ret += x*day++
            if(values[i].isNotEmpty()) {
                q.add(Pair(values[i].removeLast(), i))
            }
        }
        return ret
    }
}