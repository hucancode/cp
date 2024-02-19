func isPowerOfTwo(n int) bool {
    return n > 0 && bits.OnesCount(uint(n)) == 1
}