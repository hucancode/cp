class Solution {
    fun frequencySort(s: String): String {
        val freq = HashMap<Char, Int>()
        val token = HashMap<Int, MutableList<Char>>()
        for(c in s) {
            val x = freq.get(c) ?: 0
            freq.put(c, x+1)
        }
        for((c, f) in freq) {
            val t = token.get(f) ?: mutableListOf<Char>()
            t.add(c)
            token.put(f, t)
        }
        return token.entries
            .sortedBy { it.key }
            .reversed()
            .joinToString("") { (f,arr) -> 
                arr.joinToString("") { it.toString().repeat(f)}
            }
    }
}