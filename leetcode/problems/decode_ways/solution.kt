class Solution {
    fun numDecodings(s: String): Int {
        val n = s.length
        val f = IntArray(n+1) {0}
        f[0] = 1
        if(s[0] != '0') {
            f[1] = 1
        }
        for(i in 2..n) {
            val a = s[i-1]
            val b = s[i-2]
            if(a != '0') {
                f[i] += f[i-1]
            }
            if(b == '1' || (b == '2' && a <= '6')) {
                f[i] += f[i-2]
            }
        }
        return f[n]
    }
}