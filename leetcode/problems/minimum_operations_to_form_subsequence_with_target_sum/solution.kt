class Solution {
    fun minOperations(nums: List<Int>, target: Int): Int {
        val count = IntArray(33) {0}
        for(x in nums) {
            val p = 31 - Integer.numberOfLeadingZeros(x)
            count[p] += 1;
        }
        var ret = 0
        val n = 32
        for(i in 0 until n) {
            if(target and (1 shl i) != 0) {
                var j = i
                while(j < n && count[j] == 0) j++
                if(j == n) return -1
                while(j > i) {
                    count[j] -= 1
                    count[j-1] += 2
                    ret++
                    j--
                }
                count[i]--
            }
            count[i+1] += count[i]/2
        }
        return ret
    }
}