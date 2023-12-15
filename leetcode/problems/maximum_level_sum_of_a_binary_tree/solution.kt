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
    fun maxLevelSum(root: TreeNode?): Int {
        var q = ArrayDeque<TreeNode>()
        q.add(root!!)
        var maxSum = Int.MIN_VALUE
        var maxLevel = 0
        var level = 0
        while(!q.isEmpty()) {
            level++
            var sum = 0
            val next = ArrayDeque<TreeNode>()
            while(!q.isEmpty()) {
                val node = q.removeFirst()
                sum += node.`val`
                node.left?.let { next.add(it) }
                node.right?.let { next.add(it) }
            }
            if(sum > maxSum) {
                maxSum = sum;
                maxLevel = level;
            }
            q = next
        }
        return maxLevel
    }
}