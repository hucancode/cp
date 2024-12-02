class Solution {
    fun checkIfExist(arr: IntArray): Boolean {
        arr.sort()
        for ((i, x) in arr.withIndex()) {
            val j = arr.binarySearch(x*2)
            if(j >= 0 && j != i) {
                return true
            }
        }
        return false
    }
}
