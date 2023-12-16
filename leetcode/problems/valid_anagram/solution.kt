class Solution {
    fun isAnagram(s: String, t: String): Boolean {
        val a = IntArray(26)
        val b = IntArray(26)
        for(c in s) a[c-'a']++
        for(c in t) b[c-'a']++
        return a contentEquals b
    }
}