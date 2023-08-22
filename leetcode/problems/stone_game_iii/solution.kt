class Solution {
    fun stoneGameIII(stoneValue: IntArray): String {
        val n = stoneValue.size
        val all = stoneValue.sum()
        val alice = IntArray(n+1) {Int.MIN_VALUE}
        val bob = IntArray(n+1) {Int.MAX_VALUE}
        alice[n] = 0
        bob[n] = 0
        for(i in n-1 downTo 0) {
            var x = 0
            for(j in 0 until Math.min(n-i,3)) {
                x += stoneValue[i+j]
                alice[i] = Math.max(alice[i], x + bob[i+j+1])
                bob[i] = Math.min(bob[i], alice[i+j+1])
            }
        }
        val a = alice[0]
        val b = all - a
        return if(a > b) "Alice" else if(a == b) "Tie" else "Bob"
    }
}