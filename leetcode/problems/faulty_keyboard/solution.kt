class Solution {
    fun finalString(s: String): String {
        var ret = ""
        for(c in s) {
            if(c == 'i') {
                ret = ret.reversed()
            } else {
                ret = ret+c
            }
        }
        return ret
    }
}