class Solution {
    fun findPaths(m: Int, n: Int, maxMove: Int, startRow: Int, startColumn: Int): Int {
        val INF = 1000_000_007
        val moves = arrayOf(Pair(-1,0), Pair(1,0), Pair(0,-1), Pair(0,1))
        var f = Array(m) { IntArray(n) {0}}
        f[startRow][startColumn] = 1
        var q = HashSet<Pair<Int,Int>>()
        q.add(Pair(startRow, startColumn))
        var ret = 0
        for(i in 0 until maxMove) {
            val f_next = Array(m) { IntArray(n) {0}}
            var q_next = HashSet<Pair<Int,Int>>()
            for((x, y) in q) {
                for((dx, dy) in moves) {
                    val nx = x + dx
                    val ny = y + dy
                    if(nx < 0 || ny < 0 || nx >= m  || ny >= n)  {
                        ret += f[x][y];
                        ret %= INF;
                    } else {
                        f_next[nx][ny] += f[x][y];
                        f_next[nx][ny] %= INF;
                        q_next.add(Pair(nx,ny));
                    }
                }
            }
            f = f_next
            q = q_next
        }
        return ret
    }
}