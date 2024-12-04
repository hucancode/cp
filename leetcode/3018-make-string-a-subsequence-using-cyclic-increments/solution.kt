class Solution {
    fun canMakeSubsequence(str1: String, str2: String): Boolean {
        val n = str1.length
        val m = str2.length
        var i = 0
        var j = 0
        while(i < n && j < m) {
            if(str1[i] == str2[j] || 
                str1[i] + 1 == str2[j] || 
                (str1[i] == 'z' && str2[j] == 'a')) {
                j++
            }
            i++
        }
        return j == m
    }
}
