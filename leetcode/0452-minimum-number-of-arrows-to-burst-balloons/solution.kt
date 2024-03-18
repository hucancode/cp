class Solution {
    fun findMinArrowShots(points: Array<IntArray>): Int {
        points.sortBy { it[0] }
        var ret = 0
        var x = Int.MAX_VALUE
        for(arr in points) {
            val a = arr[0]
            val b = arr[1]
            if(a > x) {
                ret++
                x = Int.MAX_VALUE
            }
            x = Math.min(x, b)
        }
        return ret + 1
    }
}
