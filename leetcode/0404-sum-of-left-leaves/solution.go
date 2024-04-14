/**
 * Definition for a binary tree node.
 * type TreeNode struct {
 *     Val int
 *     Left *TreeNode
 *     Right *TreeNode
 * }
 */
func sum(root *TreeNode, take bool) int {
    if root == nil {
        return 0
    }
    if root.Left == nil && root.Right == nil && take {
        return root.Val
    }
    return sum(root.Left, true) + sum(root.Right, false)
}
func sumOfLeftLeaves(root *TreeNode) int {
    return sum(root, false)
}
