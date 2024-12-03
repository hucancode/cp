class Solution {
    fun addSpaces(s: String, spaces: IntArray): String {
        val builder = StringBuilder(s.length + spaces.size)
        var i = 0
        for(j in spaces) {
            builder.append(s, i, j)
            builder.append(' ')
            i = j
        }
        builder.append(s, i, s.length)
        return builder.toString()
    }
}
