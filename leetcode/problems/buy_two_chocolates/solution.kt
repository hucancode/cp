class Solution {
    fun buyChoco(prices: IntArray, money: Int): Int {
        var x = 5000
        var y = 5000
        for(p in prices) {
            y = Math.min(y, Math.max(x, p))
            x = Math.min(x, p)
        }
        return if(money >= x+y) { money - x - y } else { money }
    }
}