class Solution {
    fun destCity(paths: List<List<String>>): String {
        val incoming = HashSet<String>()
        val outgoing = HashSet<String>()
        for(uv in paths) {
            outgoing.add(uv[0])
            incoming.add(uv[1])
        }
        return incoming.minus(outgoing).first()
    }
}