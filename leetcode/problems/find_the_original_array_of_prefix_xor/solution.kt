class Solution {
    fun findArray(pref: IntArray): IntArray {
      for(i in pref.size - 1 downTo 1) {
        pref[i] = pref[i] xor pref[i-1]
      }
      return pref
    }
}