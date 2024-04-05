class Solution {
    fun optimalDivision(nums: IntArray): String {
        if(nums.size < 3) {
            return nums.joinToString("/")
        }
        return nums.first().toString() +
            nums.drop(1).joinToString("/","/(", ")")       
    }
}
