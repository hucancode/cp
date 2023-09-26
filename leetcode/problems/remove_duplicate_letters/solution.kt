class Solution {
    fun removeDuplicateLetters(s: String): String {
        val count = IntArray(26) {0}
        val vis = BooleanArray(26) {false}
        val input = s.toByteArray(Charsets.UTF_8)
        for(c in input) count[c-'a'.toByte()]++
        var ret = ArrayDeque<Byte>()
        for(c in input) {
            val i = c - 'a'.toByte()
            count[i]--
            if(vis[i]) continue;
            while(!ret.isEmpty()) {
                val j = ret.last() - 'a'.toInt()
                if(i > j || count[j] <= 0) break
                vis[j] = false
                ret.removeLast()
            }
            ret.add(c)
            vis[i] = true
        }
        return ret.toByteArray().decodeToString()
    }
}