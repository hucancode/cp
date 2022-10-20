/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode(int x) : val(x), left(NULL), right(NULL) {}
 * };
 */
class Solution {
public:
    TreeNode* dfs(TreeNode* root, TreeNode* p, TreeNode* q) {
        // if found p or q in both leaf, return root
        // if found p or q in only 1 left, return that leaf
        // else return null
        if(!root) {
            return nullptr;
        }
        if(root == p) {
            return p;
        }
        if(root == q) {
            return q;
        }
        auto left = dfs(root->left, p, q);
        auto right = dfs(root->right, p, q);
        if(left && right) {
            return root;
        }
        if(left) {
            return left;
        }
        if(right) {
            return right;
        }
        return nullptr;
    }
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        return dfs(root, p, q);
    }
};