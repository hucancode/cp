class Solution {
    fun numBusesToDestination(routes: Array<IntArray>, source: Int, target: Int): Int {
        if(source == target) {
            return 0
        }
        val n = routes.size
        val routes = routes.map { it.toSet() }
        val adj = Array(n) {mutableListOf<Int>()}
        for(i in 0 until n) {
            for(j in i+1 until n) {
                if(routes[i].any {it in routes[j]}) {
                    adj[i].add(j)
                    adj[j].add(i)
                }
            }
        }
        val q = LinkedList<Pair<Int,Int>>(routes.withIndex()
            .filter { (i,route) -> route.contains(source) }
            .map { (i,route) -> Pair(i, 1) })
        val vis = BooleanArray(n) {false}
        while(!q.isEmpty()) {
            val (u, du) = q.remove()
            if(vis[u]) {
                continue
            }
            vis[u] = true
            if(routes[u].contains(target)) {
                return du
            }
            for(v in adj[u]) {
                q.add(Pair(v, du+1))
            }
        }
        return -1
    }
}