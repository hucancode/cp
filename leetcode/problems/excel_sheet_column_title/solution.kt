class Solution {
    fun convertToTitle(columnNumber: Int): String {
        val k = 26
        val offset = 'A'.toInt()
        var ret = ""
        var n = columnNumber - 1
        while(true) {
            ret += (n%k + offset).toChar()
            if(n < k) {
                break
            }
            n = n/k - 1
        }
        return ret.reversed()
    }
}