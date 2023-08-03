class Solution {
    fun groupAnagrams(strs: Array<String>): List<List<String>> {
        val f = mutableMapOf<MutableList<Int>, MutableList<String>>()
        for(s in strs) {
            var key = MutableList(26) { 0 }
            for(i in s.map {it - 'a'}) {
                key[i]++
            }
            f.getOrPut(key, { mutableListOf<String>() })
                .add(s)
        }
        return f.values.toList()
    }
}