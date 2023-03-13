/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    TreeNode* sortedListToBST(ListNode* head) {
        if(!head) {
            return nullptr;
        }
        ListNode* prev = nullptr;
        auto turtle = head;
        auto hare = head;
        while(hare) {
            hare = hare->next;
            if(!hare) break;
            hare = hare->next;
            prev = turtle;
            turtle = turtle->next;
        }
        if(prev) {
            prev->next = nullptr;
        }
        auto root = new TreeNode(turtle->val);
        if(turtle != head) {
            root->left = sortedListToBST(head);
        }
        root->right = sortedListToBST(turtle->next);
        return root;
    }
};