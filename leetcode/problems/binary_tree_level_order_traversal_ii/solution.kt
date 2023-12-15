/**
 * Example:
 * var ti = TreeNode(5)
 * var v = ti.`val`
 * Definition for a binary tree node.
 * class TreeNode(var `val`: Int) {
 *     var left: TreeNode? = null
 *     var right: TreeNode? = null
 * }
 */
class Solution {
    fun levelOrderBottom(root: TreeNode?): List<List<Int>> {
        var q = ArrayDeque<TreeNode>()
        root?.let { q.add(it) }
        val ret = mutableListOf<List<Int>>()
        while(!q.isEmpty()) {
            val v = mutableListOf<Int>()
            val next = ArrayDeque<TreeNode>()
            while(!q.isEmpty()) {
                val node = q.removeFirst()
                v += node.`val`
                node.left?.let { next.add(it) }
                node.right?.let { next.add(it) }
            }
            ret += v
            q = next
        }
        return ret.reversed()
    }
}