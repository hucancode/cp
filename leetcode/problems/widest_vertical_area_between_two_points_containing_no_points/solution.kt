class Solution {
    fun maxWidthOfVerticalArea(points: Array<IntArray>): Int {
        points.sortBy { it[0] }
        return points
            .asSequence()
            .windowed(2)
            .map { it[1][0] - it[0][0] }
            .max()
    }
}