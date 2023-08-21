class Solution {
    fun repeatedSubstringPattern(s: String): Boolean {
        val n = s.length
        if(n < 2) {
            return false
        }
        val primes = BooleanArray(n+1) {true}
        for(i in 2..n) {
            if(primes[i]) {
                var j = 2
                while(i*j <= n) primes[i*j++] = false
            }
        }
        for(x in primes.withIndex()
            .drop(2)
            .filter{(i,x) -> x}
            .map{(i,x) -> i}
            .filter{n%it == 0}) {
                val len = n/x
                var good = true
                for(i in 0 until len) {
                    for(j in 1 until x) {
                        if(s[i] != s[i+j*len]) {
                            good = false;
                            break;
                        }
                    }
                }
                if(good) {
                    return true
                }
            }
        return false
    }
}