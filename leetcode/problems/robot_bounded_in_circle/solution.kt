class Solution {
    fun isRobotBounded(instructions: String): Boolean {
        var dx = 0
        var dy = 0
        var dr = 0
        for(c in instructions)
            when(c) {
                'L' -> dr = (dr+3)%4
                'R' -> dr = (dr+1)%4
                else -> {
                    when(dr) {
                        0 -> dy += 1
                        1 -> dx += 1
                        2 -> dy -= 1
                        else -> dx -= 1
                    }
                }
            }
        val no_translation = dx == 0 && dy == 0
        val has_rotation = dr != 0
        return no_translation || has_rotation
    }
}