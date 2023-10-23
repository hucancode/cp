class Solution {
    fun isPowerOfFour(n: Int): Boolean {
        return n.countOneBits() == 1 && 
            n.countLeadingZeroBits() % 2 == 1
    }
}