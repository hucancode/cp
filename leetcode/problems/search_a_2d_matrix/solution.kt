class Solution {
    fun searchMatrix(matrix: Array<IntArray>, target: Int): Boolean {
        var i = matrix.map { it.first() }.binarySearch(target)
        val notFound = i < 0
        if(notFound) {
            val insertPoint = -(i + 1)
            i = insertPoint - 1
        }
        if(i < 0) {
            return false
        }
        val j = matrix[i].binarySearch(target)
        return j >= 0
    }
}