class Solution {
    fun isIsomorphic(s: String, t: String): Boolean {
        val n = s.length
        val st = HashMap<Char, Char>()
        val ts = HashMap<Char, Char>()
        for((a,b) in s.zip(t)) {
            if((st.containsKey(a) && st[a] != b) ||
            (ts.containsKey(b) && ts[b] != a)) {
                return false
            } else {
                st.put(a, b)
                ts.put(b, a)
            }
        }
        return true
    }
}