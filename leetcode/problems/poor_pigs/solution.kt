class Solution {
    fun poorPigs(buckets: Int, minutesToDie: Int, minutesToTest: Int): Int {
        val test = minutesToTest/minutesToDie+1
        return ceil(log10(buckets.toDouble())/log10(test.toDouble())).toInt()
    }
}