class Solution {
    fun kthGrammar(n: Int, k: Int): Int {
        return (k-1).countOneBits() % 2
    }
}