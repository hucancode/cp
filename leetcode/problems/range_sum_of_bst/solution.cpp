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
    int travel(TreeNode* root, int low, int high) {
        if(!root) {
            return 0;
        }
        if(root->val > high) {
            return travel(root->left, low, high);
        }
        if(root->val < low) {
            return travel(root->right, low, high);
        }
        return root->val + travel(root->left, low, high) + travel(root->right, low, high);
    }
    int rangeSumBST(TreeNode* root, int low, int high) {
        return travel(root, low, high);
    }
};