class Solution {
    fun reorganizeString(s: String): String {
        val count = HashMap<Char, Int>()
        val pool = PriorityQueue<Pair<Int, Char>>(compareByDescending { it.first })
        for(c in s) {
            val x = count.get(c) ?: 0
            count.put(c, x+1)
        }
        for((k,v) in count.entries) {
            pool.add(Pair(v, k))
        }
        var ret = ""
        var cx = 0
        var x = '.'
        while(!pool.isEmpty()) {
            var (cy, y) = pool.remove()
            ret += y
            if(cx > 0) {
                pool.add(Pair(cx, x))
            }
            cx = cy - 1
            x = y
        }
        if(cx > 0) {
            ret = ""
        }
        return ret
    }
}