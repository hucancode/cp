class Solution {
    fun maximumSwap(num: Int): Int {
        val a = mutableListOf<Int>()
        var n = num
        while(n != 0) {
            a.add(n%10)
            n /= 10
        }
        for(i in (0 until a.size).reversed()) {
            var k = i
            for(j in 0 until i) if(a[j] > a[k]) k = j
            Collections.swap(a, k, i)
            if(k != i) break
        }
        return a.reversed().reduce { acc, x -> acc*10+x }
    }
}