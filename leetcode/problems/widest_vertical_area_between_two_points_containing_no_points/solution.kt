class Solution {
    fun maxWidthOfVerticalArea(points: Array<IntArray>): Int {
        return points
            .map { it[0] }
            .sorted()
            .zipWithNext { a, b -> b - a }
            .max()
    }
}