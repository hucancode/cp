/**
 * Example:
 * var ti = Node(5)
 * var v = ti.`val`
 * Definition for a Node.
 * class Node(var `val`: Int) {
 *     var next: Node? = null
 *     var random: Node? = null
 * }
 */

class Solution {
    fun copyRandomList(head: Node?): Node? {
        val mp = mutableMapOf<Node?, Node?>()
        val clone = { node: Node? ->
            node?.let { mp.getOrPut(it) {Node(it.`val`)} }
        }
        var node = head
        while(node != null) {
            clone(node)?.next = clone(node.next)
            clone(node)?.random = clone(node.random)
            node = node?.next
        }
        return mp.get(head)
    }
}