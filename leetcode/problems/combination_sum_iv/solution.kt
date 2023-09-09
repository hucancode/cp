class Solution {
    fun combinationSum4(nums: IntArray, target: Int): Int {
        val f = IntArray(target+1) {0}
        f[0] = 1
        for(i in 1..target) {
            for(j in nums) {
                if(j <= i) {
                    f[i] += f[i-j]
                }
            }
        }
        return f[target]
    }
}