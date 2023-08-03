class Solution {
    fun smallestSufficientTeam(req_skills: Array<String>, people: List<List<String>>): IntArray {
        var people = people.map { it.fold(0) { acc, next -> 
                acc or (1 shl req_skills.indexOf(next))
            }
        }
        var n = req_skills.size
        val m = 1 shl n
        n = people.size
        var f = List(m, { MutableList(n, {it}) })
        f[0].clear()
        for (mask in 0 until m) {
            for (i in 0 until n) {
                val next = mask or people[i]
                if (f[next].size <= f[mask].size + 1) {
                    continue
                }
                f[next].clear()
                f[next].addAll(f[mask])
                f[next].add(i)
            }
        }
        return f.last().toIntArray()
    }
}