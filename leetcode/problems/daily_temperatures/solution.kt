class Solution {
    fun dailyTemperatures(temperatures: IntArray): IntArray {
        val n = temperatures.size
        val ret = IntArray(n) {0}
        val st = ArrayDeque<Pair<Int,Int>>()
        for((i, x) in temperatures.withIndex()) {
            while(!st.isEmpty()) {
                val (j, y) = st.last()
                if(x > y) {
                    st.removeLast()
                    ret[j] = i-j
                } else {
                    break;
                }
            }
            st.add(Pair(i, x))
        }
        return ret
    }
}