class Solution {
    fun candy(ratings: IntArray): Int {
        val n = ratings.size
        val candy = IntArray(n) {1}
        ratings.indices
            .zipWithNext()
            .plus(ratings.indices
                .reversed()
                .zipWithNext())
            .filter { (a,b) -> ratings[b] > ratings[a] }
            .forEach { (a,b) -> candy[b] = Math.max(candy[b], candy[a]+1)}
        return candy.sum()
    }
}