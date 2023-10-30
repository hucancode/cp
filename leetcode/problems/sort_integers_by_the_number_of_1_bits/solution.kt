class Solution {
    fun sortByBits(arr: IntArray): IntArray {
      return arr.toList()
        .sortedWith(compareBy<Int>{ it.countOneBits() }.thenBy{ it })
        .toIntArray()
    }
}