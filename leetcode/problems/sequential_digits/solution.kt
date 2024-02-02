class Solution {
    fun sequentialDigits(low: Int, high: Int): List<Int> {
        val ret = MutableList<Int>(0) {0}
        for(len in 1..9)
            for(i in 1..(10-len)) {
                var k = (i until i+len).fold(0) { acc, x -> acc*10 + x }
                if(k >= low && k <= high) ret.add(k)
            }
        return ret
    }
}