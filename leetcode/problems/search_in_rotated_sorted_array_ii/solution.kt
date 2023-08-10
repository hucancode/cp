class Solution {
    fun searchDistinct(nums: IntArray, target: Int): Int {
        var n = nums.size
        var l = 0
        var r = n-1
        while(l < r) {
            val m = (l+r)/2
            val good = nums[m] < nums[r]
            if(good) {
                r = m
            } else {
                l = m+1
            }
        }
        if(target > nums[n-1]) {
            r = if(l == 0) n-1 else l-1
            l = 0
        } else {
            r = n-1
        }
        while(l < r) {
            val m = (l+r)/2
            val good = nums[m] >= target
            if(good) {
                r = m
            } else {
                l = m+1
            }
        }
        if(nums[l] != target) {
            return -1
        }
        return l
    }
    fun dedupe(nums: IntArray): IntArray {
        var k = 0
        for (i in nums.indices) {
            if (i == 0 || nums[i] != nums[i - 1]) {
                nums[k++] = nums[i]
            }
        }
        return nums.copyOf(k)
    }
    fun search(nums: IntArray, target: Int): Boolean {
        return searchDistinct(dedupe(nums), target) != -1
    }
}