/**
 * Definition for singly-linked list.
 * type ListNode struct {
 *     Val int
 *     Next *ListNode
 * }
 */
func mergeInBetween(list1 *ListNode, a int, b int, list2 *ListNode) *ListNode {
    node_a := list1
    i := 1
    for i < a {
        node_a = node_a.Next
        i++
    }
    node_b := node_a.Next
    for i <= b {
        node_b = node_b.Next
        i++
    }
    node_a.Next = list2
    for node_a.Next != nil {
        node_a = node_a.Next
    }
    node_a.Next = node_b
    return list1
}
