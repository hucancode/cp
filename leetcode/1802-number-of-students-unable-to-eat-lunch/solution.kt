class Solution {
    fun countStudents(students: IntArray, sandwiches: IntArray): Int {
        var a = students.sum()
        var b = students.size - a
        for(x in sandwiches) {
            if(x == 1 && a > 0) {
                a--
            } else if(x == 0 && b > 0) {
                b--
            } else {
                break
            }
        }
        return a + b
    }
}
