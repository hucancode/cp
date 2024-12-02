class Solution {
    fun isPrefixOfWord(sentence: String, searchWord: String): Int {
        val i = sentence
            .splitToSequence(' ')
            .indexOfFirst { it.startsWith(searchWord) }
        if(i >= 0) {
            return i+1
        }
        return i
    }
}
