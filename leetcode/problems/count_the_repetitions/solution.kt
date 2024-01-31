class Solution {
    fun getMaxRepetitions(s1: String, n1: Int, s2: String, n2: Int): Int {
        val n = s1.length
        val m = s2.length
        val f = IntArray(m)
        for(j in 0 until m) {
            var k = 0
            for(i in 0 until n) if(s1[i] == s2[(j+k)%m]) k++
            f[j] = k
        }
        var j = 0
        for(i in 0 until n1) j += f[j%m]
        return j/m/n2
    }
}