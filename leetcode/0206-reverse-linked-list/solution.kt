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
    fun reverseList(head: ListNode?): ListNode? {
        if(head == null) {
            return head
        }
        var ret = head
        var node = ret?.next
        ret?.next = null
        while(node != null) {
            val new_node = node?.next
            node?.next = ret
            ret = node
            node = new_node
        }
        return ret
    }
}
