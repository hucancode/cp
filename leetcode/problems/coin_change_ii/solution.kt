class Solution {
    fun change(amount: Int, coins: IntArray): Int {
        val f = Array(amount+1) { IntArray(coins.size+1) {0} }
        f[0][0] = 1
        for(x in 0..amount) {
            for(i in 1..coins.size) {
                val y = x - coins[i-1]
                if(y < 0) continue
                for(j in 0..i) {
                    f[x][i] += f[y][j]
                }
            }
        }
        //println(f.joinToString())
        return f[amount].sum()
    }
}