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
    fun hasCycle(head: ListNode?): Boolean {
        var rabbit = head
        var turtle = head
        while(rabbit != null) {
            repeat(2) {
                rabbit = rabbit?.next
                if(rabbit == turtle) return true
            }
            turtle = turtle?.next
        }
        return false
    }
}