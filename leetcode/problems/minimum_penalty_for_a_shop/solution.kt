class Solution {
    fun bestClosingTime(customers: String): Int {
        val n = customers.length
        val emptyLeft = IntArray(n+1) {0}
        val customerRight = IntArray(n+1) {0}
        for(i in 1..n) {
            emptyLeft[i] = emptyLeft[i-1] + if(customers[i-1] == 'N') 1 else 0
        }
        for(i in n-1 downTo 0) {
            customerRight[i] = customerRight[i+1] + if(customers[i] == 'Y') 1 else 0
        }
        var h = 0;
        var penalty = emptyLeft[0] + customerRight[0]
        for(i in 0..n) {
            val next = emptyLeft[i] + customerRight[i]
            if(next < penalty) {
                penalty = next;
                h = i
            }
        }
        return h
    }
}