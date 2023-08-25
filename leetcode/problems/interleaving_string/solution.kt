class Solution {
    fun isInterleave(s1: String, s2: String, s3: String): Boolean {
        val n = s1.length
        val m = s2.length
        val nm = s3.length
        if(n+m != nm) {
            return false
        }
        val f = Array(n+1) {BooleanArray(m+1) {false}}
        for(i in 0..n) {
            for(j in 0..m) {
                if(i == 0 && j == 0) {
                    f[i][j] = true
                    continue;
                }
                if(i > 0) {
                    val matched = s1[i-1] == s3[i+j-1]
                    f[i][j] = f[i][j] || (f[i-1][j] && matched)
                }
                if(j > 0) {
                    val matched = s2[j-1] == s3[i+j-1]
                    f[i][j] = f[i][j] || (f[i][j-1] && matched)
                }
            }
        }
        return f[n][m]
    }
}