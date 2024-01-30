class Solution {
    fun evalRPN(tokens: Array<String>): Int {
        val st = ArrayDeque<Int>()
        for(s in tokens) {
            when(s) {
                "+" -> {
                    val b = st.removeLast()
                    val a = st.removeLast()
                    st.add(a+b)
                }
                "-" -> {
                    val b = st.removeLast()
                    val a = st.removeLast()
                    st.add(a-b)
                }
                "*" -> {
                    val b = st.removeLast()
                    val a = st.removeLast()
                    st.add(a*b)
                }
                "/" -> {
                    val b = st.removeLast()
                    val a = st.removeLast()
                    st.add(a/b)
                }
                else -> {
                    st.add(s.toInt())
                }
            }
        }
        return st.removeLast()
    }
}