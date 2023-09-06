/**
 * Example:
 * var li = ListNode(5)
 * var v = li.`val`
 * Definition for singly-linked list.
 * class ListNode(var `val`: Int) {
 *     var next: ListNode? = null
 * }
 */
class Solution {
    fun splitListToParts(head: ListNode?, k: Int): Array<ListNode?> {
        var n = 0
        var node = head
        while(node != null) {
            node = node?.next
            n++
        }
        var remainder = n%k
        val len = n/k
        val ret = MutableList<ListNode?>(0) {null}
        node = head
        var tail = node
        repeat(k) {
            ret.add(node)
            var n = len
            if(remainder > 0) {
                remainder--
                n++
            }
            repeat(n) {
                tail = node
                node = node?.next
            }
            tail?.next = null
        }
        return ret.toTypedArray()
    }
}